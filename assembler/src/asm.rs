use std::{collections::HashMap, vec};

use crate::ast::*;

#[derive(Debug)]
pub enum AssemblerError {
    UndefinedLabel(String),
    AddressOutOfBounds26(u32),
    AddressOutOfBounds16(u32),
}

pub fn assemble(items: &[Item], insert_nops: bool) -> Result<Vec<u8>, AssemblerError> {
    // First pass: collect label positions and expand pseudo-instructions
    let mut label_positions: HashMap<String, u32> = HashMap::new();
    let mut expanded_items: Vec<Item> = Vec::new();
    let mut current_pc: u32 = 0;

    for item in items {
        match item {
            Item::Label(name) => {
                label_positions.insert(name.clone(), current_pc);
            }
            Item::Instruction(inst) => {
                let expanded = expand_pseudo_instruction(inst, current_pc, insert_nops);
                for expanded_inst in expanded {
                    expanded_items.push(Item::Instruction(expanded_inst));
                    current_pc += 1;
                }
            }
        }
    }

    // Second pass: emit bytecode with resolved labels
    let mut bytecode = Vec::new();

    for item in &expanded_items {
        if let Item::Instruction(inst) = item {
            emit_instruction(inst, &label_positions, &mut bytecode)?;
        }
    }

    Ok(bytecode)
}

fn is_pseudo_instruction(inst: &Instruction) -> bool {
    matches!(
        inst,
        Instruction::Mov { .. }
            | Instruction::Copy { .. }
            | Instruction::Push { .. }
            | Instruction::Pop { .. }
            | Instruction::Call { .. }
            | Instruction::PushPc { .. }
            | Instruction::Ret
    )
}

fn expand_pseudo_instruction(
    inst: &Instruction,
    current_pc: u32,
    insert_nops: bool,
) -> Vec<Instruction> {
    let expanded = match inst {
        // Mov: load 32-bit immediate into register using shi/slo
        Instruction::Mov { dst, imm } => {
            let hi = (*imm >> 16) as u16;
            let lo = (*imm & 0xFFFF) as u16;
            vec![
                Instruction::Shi { dst: *dst, imm: hi },
                Instruction::Slo { dst: *dst, imm: lo },
            ]
        }
        // Copy: or dst, src, $0 (assuming $0 is zero register)
        Instruction::Copy { dst, src } => {
            vec![Instruction::Arith {
                op: ArithOp::Or,
                dst: *dst,
                src1: *src,
                src2: 0,
            }]
        }
        // Push: store src to stack pointer and decrement
        Instruction::Push { src } => {
            vec![
                Instruction::Mov { dst: 29, imm: 1 },
                Instruction::Store { dst: 31, src: *src },
                Instruction::Arith {
                    op: ArithOp::Sub,
                    dst: 31,
                    src1: 31,
                    src2: 29,
                },
            ]
        }
        // Pop: increment stack pointer and load into dst
        Instruction::Pop { dst } => {
            vec![
                Instruction::Mov { dst: 29, imm: 1 },
                Instruction::Arith {
                    op: ArithOp::Add,
                    dst: 31,
                    src1: 31,
                    src2: 29,
                },
                Instruction::Load { dst: *dst, src: 31 },
            ]
        }
        // Call: push return address and jump
        Instruction::Call { label } => {
            vec![
                Instruction::PushPc { offset: 1 },
                Instruction::JumpLabel {
                    label: label.clone(),
                },
            ]
        }
        // Ret: pop return address and jump
        Instruction::Ret => {
            vec![
                Instruction::Pop { dst: 30 },
                Instruction::JumpRegister { target: 30 },
            ]
        }
        // PushPc: push the program counter to the stack
        Instruction::PushPc { offset } => {
            let pc = if insert_nops {
                current_pc + (6 + offset) * 5
            } else {
                current_pc + 6 + offset
            };

            vec![
                Instruction::Mov { dst: 30, imm: pc },
                Instruction::Push { src: 30 },
            ]
        }
        // All other instructions are already real
        _ => {
            vec![inst.clone()]
        }
    };

    // Recursively expand any pseudo-instructions in the result
    let mut fully_expanded = Vec::new();

    for instr in expanded {
        if is_pseudo_instruction(&instr) {
            fully_expanded.extend(expand_pseudo_instruction(
                &instr,
                current_pc + fully_expanded.len() as u32,
                insert_nops,
            ));
        } else {
            if insert_nops {
                fully_expanded.push(Instruction::Nop);
                fully_expanded.push(Instruction::Nop);
                fully_expanded.push(Instruction::Nop);
                fully_expanded.push(Instruction::Nop);
            }

            fully_expanded.push(instr);
        }
    }

    fully_expanded
}

const OP_ARITH: u32 = 0x00;
const OP_SHI: u32 = 0x01;
const OP_SLO: u32 = 0x02;
const OP_LOAD: u32 = 0x03;
const OP_STORE: u32 = 0x04;
const OP_BRANCH: u32 = 0x05;
const OP_JUMP_REG: u32 = 0x06;
const OP_JUMP_IMM: u32 = 0x07;
const OP_HALT: u32 = 0x3E;
const OP_NOP: u32 = 0x3F;

fn emit_instruction(
    inst: &Instruction,
    labels: &HashMap<String, u32>,
    output: &mut Vec<u8>,
) -> Result<(), AssemblerError> {
    // All instructions are encoded as 32 bits (4 bytes)
    let instr_word: u32 = match inst {
        Instruction::Arith {
            op,
            dst,
            src1,
            src2,
        } => {
            let funct = match op {
                ArithOp::Add => 0x00,
                ArithOp::Sub => 0x01,
                ArithOp::And => 0x02,
                ArithOp::Or => 0x03,
                ArithOp::Xor => 0x04,
                ArithOp::Not => 0x09,
                ArithOp::Lts => 0x0A,
                ArithOp::Gts => 0x0B,
                ArithOp::Ltu => 0x0C,
                ArithOp::Gtu => 0x0D,
                ArithOp::Eq => 0x0E,
                ArithOp::Ne => 0x0F,
            };

            OP_ARITH << 26
                | (*src1 as u32) << 21
                | (*src2 as u32) << 16
                | (*dst as u32) << 11
                | funct
        }
        Instruction::Halt => OP_HALT << 26,
        Instruction::Nop => OP_NOP << 26,
        Instruction::Shi { dst, imm } => OP_SHI << 26 | (*dst as u32) << 16 | (*imm as u32),
        Instruction::Slo { dst, imm } => OP_SLO << 26 | (*dst as u32) << 16 | (*imm as u32),
        Instruction::JumpLabel { label } => {
            let instr_addr = *labels
                .get(label)
                .ok_or_else(|| AssemblerError::UndefinedLabel(label.clone()))?;

            if instr_addr > 0x3FFFFFF {
                return Err(AssemblerError::AddressOutOfBounds26(instr_addr));
            }

            OP_JUMP_IMM << 26 | (instr_addr & 0x3FFFFFF)
        }
        Instruction::JumpRegister { target } => OP_JUMP_REG << 26 | (*target as u32) << 21,
        Instruction::Branch { cond, label } => {
            let instr_addr = *labels
                .get(label)
                .ok_or_else(|| AssemblerError::UndefinedLabel(label.clone()))?;

            if instr_addr > 0xFFFF {
                return Err(AssemblerError::AddressOutOfBounds16(instr_addr));
            }

            OP_BRANCH << 26 | (*cond as u32) << 21 | (instr_addr & 0xFFFF)
        }
        Instruction::Load { dst, src } => OP_LOAD << 26 | (*src as u32) << 21 | (*dst as u32) << 16,
        Instruction::Store { dst, src } => {
            OP_STORE << 26 | (*src as u32) << 21 | (*dst as u32) << 16
        }
        _ => unreachable!("Pseudo-instructions should be expanded"),
    };

    output.extend(instr_word.to_be_bytes());
    Ok(())
}
