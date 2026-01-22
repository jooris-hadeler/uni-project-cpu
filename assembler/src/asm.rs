use std::collections::HashMap;

use crate::ast::*;

const OP_ARITH: u32 = 0x00;
const OP_SHI: u32 = 0x01;
const OP_SLO: u32 = 0x02;
const OP_LOAD: u32 = 0x03;
const OP_STORE: u32 = 0x04;
const OP_BR: u32 = 0x05;
const OP_JR: u32 = 0x06;
const OP_JMP: u32 = 0x07;
const OP_JAR: u32 = 0x08;
const OP_NOP: u32 = 0x3F;

const ARITH_ADD: u32 = 0x00;
const ARITH_SUB: u32 = 0x01;
const ARITH_AND: u32 = 0x02;
const ARITH_OR: u32 = 0x03;
const ARITH_XOR: u32 = 0x04;
const ARITH_SHL: u32 = 0x05;
const ARITH_SHR: u32 = 0x07;
const ARITH_SAR: u32 = 0x08;
const ARITH_NOT: u32 = 0x09;
const ARITH_LTS: u32 = 0x0A;
const ARITH_GTS: u32 = 0x0B;
const ARITH_LTU: u32 = 0x0C;
const ARITH_GTU: u32 = 0x0D;
const ARITH_EQ: u32 = 0x0E;
const ARITH_NE: u32 = 0x0F;

const fn construct_r(op: u32, rs: u8, rt: u8, rd: u8, funct: u32) -> u32 {
    assert!(op <= 0x3F);
    assert!(rs <= 0x1F);
    assert!(rt <= 0x1F);
    assert!(rd <= 0x1F);
    assert!(funct <= 0x3F);

    op << 26 | (rs as u32) << 21 | (rt as u32) << 16 | (rd as u32) << 11 | funct
}

const fn construct_i(op: u32, rs: u8, rt: u8, imm: u16) -> u32 {
    assert!(op <= 0x3F);
    assert!(rs <= 0x1F);
    assert!(rt <= 0x1F);

    op << 26 | (rs as u32) << 21 | (rt as u32) << 16 | (imm as u32)
}

const fn construct_j(op: u32, addr: u32) -> u32 {
    assert!(op <= 0x3F);
    assert!(addr <= 0x3FFFFFF);

    op << 26 | addr
}

const fn patch_i(inst: u32, imm: i16) -> u32 {
    let inst = inst & 0xFFFF0000;

    inst | ((imm as u16) as u32)
}

const fn patch_j(inst: u32, addr: u32) -> u32 {
    assert!(addr <= 0x3FFFFFF);

    let inst = inst & 0xFC000000;

    inst | addr
}

struct Patch {
    /// The index in the output `Vec<u32>` where the instruction is stored.
    code_index: usize,
    /// The Instruction Pointer index where this instruction is located.
    address_idx: u32,
    /// The label to resolve.
    label: String,
    /// The kind of patch to apply.
    kind: PatchKind,
}

enum PatchKind {
    Relative16,
    Relative26,
}

pub struct Assembler {
    /// The final machine code
    code: Vec<u32>,
    /// Map of Label Name -> Instruction Index (Word Address)
    symbol_table: HashMap<String, u32>,
    /// List of instructions that need patching after the first pass
    patches: Vec<Patch>,
    /// Tracks the current Instruction Index (0, 1, 2...)
    current_idx: u32,
    /// Should the assembler emit nops between instructions
    should_emit_nops: bool,
}

impl Assembler {
    pub fn new(should_emit_nops: bool) -> Self {
        Self {
            code: Vec::new(),
            symbol_table: HashMap::new(),
            patches: Vec::new(),
            current_idx: 0,
            should_emit_nops,
        }
    }

    pub fn assemble(mut self, items: &[Item]) -> Result<Vec<u32>, String> {
        // --- STAGE 1 & 2: Expansion & Emission ---
        for item in items {
            match item {
                Item::Label(name) => {
                    if self.symbol_table.contains_key(name) {
                        return Err(format!("Duplicate label definition: {}", name));
                    }
                    // Map label to the current instruction index
                    self.symbol_table.insert(name.clone(), self.current_idx);
                }
                Item::Real(inst) => self.process_real(inst),
                Item::Pseudo(inst) => self.process_pseudo(inst),
            }
        }

        // --- STAGE 3: Patching ---
        self.apply_patches()?;

        Ok(self.code)
    }

    fn emit(&mut self, instruction: u32) {
        self.code.push(instruction);
        self.current_idx += 1;
    }

    fn emit_nops(&mut self) {
        for _ in 0..4 {
            self.emit(construct_j(OP_NOP, 0));
        }
    }

    fn emit_optional_nops(&mut self) {
        if self.should_emit_nops {
            self.emit_nops();
        }
    }

    fn process_real(&mut self, inst: &RealInstruction) {
        match inst {
            RealInstruction::Arith { op, rd, rs, rt } => {
                let funct = match op {
                    ArithOp::Add => ARITH_ADD,
                    ArithOp::Sub => ARITH_SUB,
                    ArithOp::And => ARITH_AND,
                    ArithOp::Or => ARITH_OR,
                    ArithOp::Xor => ARITH_XOR,
                    ArithOp::Shl => ARITH_SHL,
                    ArithOp::Shr => ARITH_SHR,
                    ArithOp::Sar => ARITH_SAR,
                    ArithOp::Not => ARITH_NOT,
                    ArithOp::Lts => ARITH_LTS,
                    ArithOp::Ltu => ARITH_LTU,
                    ArithOp::Gts => ARITH_GTS,
                    ArithOp::Gtu => ARITH_GTU,
                    ArithOp::Eq => ARITH_EQ,
                    ArithOp::Ne => ARITH_NE,
                };

                self.emit(construct_r(OP_ARITH, *rs, *rt, *rd, funct));
                self.emit_optional_nops();
            }
            RealInstruction::Shi { rs, rt, imm } => {
                self.emit(construct_i(OP_SHI, *rs, *rt, *imm));
                self.emit_optional_nops();
            }
            RealInstruction::Slo { rs, rt, imm } => {
                self.emit(construct_i(OP_SLO, *rs, *rt, *imm));
                self.emit_optional_nops();
            }
            RealInstruction::Load { rs, rt, imm } => {
                self.emit(construct_i(OP_LOAD, *rs, *rt, *imm as u16));
                self.emit_optional_nops();
            }
            RealInstruction::Store { rs, rt, imm } => {
                self.emit(construct_i(OP_STORE, *rs, *rt, *imm as u16));
                self.emit_optional_nops();
            }
            RealInstruction::Jr { rs } => {
                self.emit(construct_r(OP_JR, *rs, 0, 0, 0));
                self.emit_nops();
            }
            RealInstruction::Nop => {
                self.emit(construct_j(OP_NOP, 0));
            }
        };
    }

    fn process_pseudo(&mut self, inst: &PseudoInstruction) {
        match inst {
            // Mov expands to Shi + Slo (2 instructions)
            PseudoInstruction::Mov { dst, imm } => {
                let upper = (imm >> 16) as u16;
                let lower = (imm & 0xFFFF) as u16;
                self.emit(construct_i(OP_SHI, 0, *dst, upper));
                self.emit_nops();
                self.emit(construct_i(OP_SLO, *dst, *dst, lower));
                self.emit_nops();
            }
            // Copy expands to OR (1 instruction)
            PseudoInstruction::Copy { dst, src } => {
                self.emit(construct_r(OP_ARITH, *src, *src, *dst, ARITH_OR));
                self.emit_nops();
            }
            // Branch to Label
            PseudoInstruction::Br { rs, rt, label } => {
                self.record_patch(label.clone(), PatchKind::Relative16);
                self.emit(construct_i(OP_BR, *rs, *rt, 0));
                self.emit_nops();
            }
            // Jump to Label
            PseudoInstruction::Jmp { label } => {
                self.record_patch(label.clone(), PatchKind::Relative26);
                self.emit(construct_j(OP_JMP, 0));
                self.emit_nops();
            }
            // Jar to Label
            PseudoInstruction::Jar { label } => {
                self.record_patch(label.clone(), PatchKind::Relative26);
                self.emit(construct_j(OP_JAR, 0));
                self.emit_nops();
            }
        }
    }

    fn record_patch(&mut self, label: String, kind: PatchKind) {
        self.patches.push(Patch {
            code_index: self.code.len(),
            address_idx: self.current_idx,
            label,
            kind,
        });
    }

    fn apply_patches(&mut self) -> Result<(), String> {
        for patch in &self.patches {
            let target_idx = *self
                .symbol_table
                .get(&patch.label)
                .ok_or_else(|| format!("Undefined label: {}", patch.label))?;

            let inst = self.code[patch.code_index];

            let new_inst = match patch.kind {
                PatchKind::Relative16 => {
                    let pc_next = patch.address_idx + 1;
                    let diff = (target_idx as i32) - (pc_next as i32);

                    // Check if offset fits in signed 16-bit integer
                    if diff < i16::MIN as i32 || diff > i16::MAX as i32 {
                        return Err(format!(
                            "16-bit offset to '{}' out of range ({})",
                            patch.label, diff
                        ));
                    }

                    patch_i(inst, diff as i16)
                }
                PatchKind::Relative26 => {
                    let pc_next = patch.address_idx + 1;
                    let diff = (target_idx as i32) - (pc_next as i32);

                    // Check if offset fits in signed 26-bit integer
                    if diff < -0x2000000 || diff > 0x1FFFFFF {
                        return Err(format!(
                            "26-bit offset to '{}' out of range ({})",
                            patch.label, diff
                        ));
                    }

                    patch_j(inst, ((diff as i64) as u32) & 0x3FFFFFF)
                }
            };

            self.code[patch.code_index] = new_inst;
        }
        Ok(())
    }
}
