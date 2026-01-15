use std::io::stdin;

use termion::input::TermRead;

use crate::isa::{self, parse_instruction};

#[derive(Default, Clone)]
pub struct IfId {
    pub empty: bool,
    pub inst: u32,
    pub next_pc: u32,
}

#[derive(Default, Clone)]
pub struct IdEx {
    pub empty: bool,
    pub op: u32,
    pub val1: u32,
    pub val2: u32,
    pub imm: u32,
    pub rd: u8,
    pub funct: u32,
}

#[derive(Default, Clone)]
pub struct ExMem {
    pub empty: bool,
    pub alu_result: u32,
    pub val2: u32, // For stores
    pub rd: u8,
    pub op: u32,
}

#[derive(Default, Clone)]
pub struct MemWb {
    pub empty: bool,
    pub alu_result: u32,
    pub mem_data: u32,
    pub rd: u8,
    pub op: u32,
}

pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32,
    pub rom: Vec<u32>,
    pub memory: Vec<u32>,

    // Pipeline Registers
    pub if_id: IfId,
    pub id_ex: IdEx,
    pub ex_mem: ExMem,
    pub mem_wb: MemWb,

    pub halted: bool,
    pub cycle_count: u64,
}

impl Cpu {
    pub fn new(rom: Vec<u32>) -> Self {
        const RAM_SIZE: usize = 1024 * 8;

        let mut cpu = Self {
            regs: [0; 32],
            pc: 0,
            rom,
            memory: vec![0; RAM_SIZE],
            if_id: IfId {
                empty: true,
                ..Default::default()
            },
            id_ex: IdEx {
                empty: true,
                ..Default::default()
            },
            ex_mem: ExMem {
                empty: true,
                ..Default::default()
            },
            mem_wb: MemWb {
                empty: true,
                ..Default::default()
            },

            halted: false,
            cycle_count: 0,
        };

        cpu.regs[31] = (RAM_SIZE - 1) as u32;

        cpu
    }

    pub fn print_assembly(&self) {
        println!("Address   Instruction");
        for (addr, word) in self.rom.iter().copied().enumerate() {
            let instr = parse_instruction(word);
            println!("{addr:>06X}    {instr}");
        }
    }

    pub fn dump(&self) {
        println!(" === REGISTER DUMP === ");

        for id in 0..32 {
            if id != 0 && id % 8 == 0 {
                println!();
            }

            print!("${id:0>2}={: <10}", self.regs[id]);
        }

        println!();
    }

    pub fn run(&mut self, single_step: bool, verbose: bool) {
        println!("Starting execution...");

        while !self.halted {
            self.step();
            self.cycle_count += 1;

            if verbose {
                let inst = isa::parse_instruction(self.if_id.inst);

                println!("{inst}");
            }

            if single_step {
                stdin().events().next();
            } else if self.cycle_count > 100_000 {
                println!("Error: Maximum cycle count exceeded.");
                break;
            }
        }

        println!("CPU Halted. Total cycles: {}", self.cycle_count);
    }

    pub fn step(&mut self) {
        if self.mem_wb.op == isa::OP_HALT {
            self.halted = true;
            return;
        }

        self.write_back();
        self.memory_access();
        self.execute();
        self.decode();
        self.fetch();
    }

    fn fetch(&mut self) {
        self.if_id = IfId {
            empty: false,
            inst: self
                .rom
                .get(self.pc as usize)
                .copied()
                .unwrap_or(isa::OP_HALT << 26),
            next_pc: self.pc + 1,
        };

        self.pc += 1;
    }

    fn decode(&mut self) {
        if self.if_id.empty {
            return;
        }

        self.id_ex = match isa::parse_instruction(self.if_id.inst) {
            isa::Instruction::R {
                op,
                rs,
                rt,
                rd,
                funct,
            } => IdEx {
                empty: false,
                op,
                val1: self.regs[rs as usize],
                val2: self.regs[rt as usize],
                imm: 0,
                rd,
                funct,
            },
            isa::Instruction::I { op, rs, rt, imm } => IdEx {
                empty: false,
                op,
                val1: self.regs[rs as usize],
                val2: self.regs[rt as usize],
                imm,
                rd: rt,
                funct: 0,
            },
            isa::Instruction::J { op, addr } => IdEx {
                empty: false,
                op,
                val1: 0,
                val2: 0,
                imm: addr,
                rd: 0,
                funct: 0,
            },
        }
    }

    fn execute(&mut self) {
        if self.id_ex.empty {
            return;
        }

        let mut result = 0;
        let mut target_pc = 0;
        let mut jump_taken = false;

        match self.id_ex.op {
            isa::OP_ARITH => {
                result = match self.id_ex.funct {
                    isa::ARITH_ADD => self.id_ex.val1.wrapping_add(self.id_ex.val2),
                    isa::ARITH_SUB => self.id_ex.val1.wrapping_sub(self.id_ex.val2),
                    isa::ARITH_AND => self.id_ex.val1 & self.id_ex.val2,
                    isa::ARITH_OR => self.id_ex.val1 | self.id_ex.val2,
                    isa::ARITH_XOR => self.id_ex.val1 ^ self.id_ex.val2,
                    isa::ARITH_NOT => !self.id_ex.val1,

                    isa::ARITH_LTS => ((self.id_ex.val1 as i32) < (self.id_ex.val2 as i32)) as u32,
                    isa::ARITH_LTU => (self.id_ex.val1 < self.id_ex.val2) as u32,
                    isa::ARITH_GTS => ((self.id_ex.val1 as i32) > (self.id_ex.val2 as i32)) as u32,
                    isa::ARITH_GTU => (self.id_ex.val1 > self.id_ex.val2) as u32,
                    isa::ARITH_EQ => (self.id_ex.val1 == self.id_ex.val2) as u32,
                    isa::ARITH_NE => (self.id_ex.val1 != self.id_ex.val2) as u32,
                    _ => 0,
                };
            }
            isa::OP_SHI => {
                result = self.id_ex.val1 | (self.id_ex.imm << 16);
            }
            isa::OP_SLO => {
                result = self.id_ex.val1 | (self.id_ex.imm & 0xFFFF);
            }
            isa::OP_LOAD => {
                result = self.id_ex.val1;
            }
            isa::OP_STORE => {
                result = self.id_ex.val2;
            }
            isa::OP_JUMP_IMM => {
                target_pc = self.id_ex.imm;
                jump_taken = true;
            }
            isa::OP_JUMP_REG => {
                target_pc = self.id_ex.val1;
                jump_taken = true;
            }
            isa::OP_BRANCH => {
                if self.id_ex.val1 & 1 != 0 {
                    target_pc = self.id_ex.imm;
                    jump_taken = true;
                }
            }
            isa::OP_NOP | isa::OP_HALT => {}
            op => unimplemented!("opcode 0x{op:X}"),
        }

        if jump_taken {
            self.pc = target_pc;
            self.flush_pipeline();
        }

        self.ex_mem = ExMem {
            empty: false,
            alu_result: result,
            val2: self.id_ex.val2,
            rd: self.id_ex.rd,
            op: self.id_ex.op,
        };
    }

    fn memory_access(&mut self) {
        if self.ex_mem.empty {
            return;
        }

        let mut mem_data = 0;
        let addr = self.ex_mem.alu_result as usize;

        if self.ex_mem.op == isa::OP_LOAD {
            mem_data = self.memory[addr];
        } else if self.ex_mem.op == isa::OP_STORE {
            self.memory[addr] = self.ex_mem.val2;
        }

        self.mem_wb = MemWb {
            empty: false,
            alu_result: self.ex_mem.alu_result,
            mem_data,
            rd: self.ex_mem.rd,
            op: self.ex_mem.op,
        };
    }

    fn write_back(&mut self) {
        if self.mem_wb.empty {
            return;
        }

        // Only write if the destination register is not R0
        // and if the operation is one that actually writes to a register.
        if self.mem_wb.rd != 0 {
            match self.mem_wb.op {
                isa::OP_ARITH | isa::OP_SHI | isa::OP_SLO | isa::OP_JUMP_IMM | isa::OP_JUMP_REG => {
                    // Arithmetic and Jump-and-Link style ops write the ALU result
                    self.regs[self.mem_wb.rd as usize] = self.mem_wb.alu_result;
                }
                isa::OP_LOAD => {
                    // Load ops write the data fetched from memory
                    self.regs[self.mem_wb.rd as usize] = self.mem_wb.mem_data;
                }
                _ => {
                    // OP_STORE, OP_BRANCH, OP_HALT, and OP_NOP do not write to registers
                }
            }
        }
    }

    fn flush_pipeline(&mut self) {
        // Clear the IF/ID and ID/EX latches
        // This effectively inserts NOPs (bubbles) into the pipeline
        self.if_id = IfId {
            empty: true,
            ..Default::default()
        };

        self.id_ex = IdEx {
            empty: true,
            ..Default::default()
        };
    }
}
