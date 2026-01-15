use std::fmt::{Display, Formatter};

pub use consts::*;

#[rustfmt::skip]
mod consts {
    pub const OP_ARITH:    u32 = 0x00;
    pub const OP_SHI:      u32 = 0x01;
    pub const OP_SLO:      u32 = 0x02;
    pub const OP_LOAD:     u32 = 0x03;
    pub const OP_STORE:    u32 = 0x04;
    pub const OP_BRANCH:   u32 = 0x05;
    pub const OP_JUMP_REG: u32 = 0x06;
    pub const OP_JUMP_IMM: u32 = 0x07;
    pub const OP_HALT:     u32 = 0x3E;
    pub const OP_NOP:      u32 = 0x3F;

    pub const ARITH_ADD:   u32 = 0x00;
    pub const ARITH_SUB:   u32 = 0x01;
    pub const ARITH_AND:   u32 = 0x02;
    pub const ARITH_OR:    u32 = 0x03;
    pub const ARITH_XOR:   u32 = 0x04;
    pub const ARITH_NOT:   u32 = 0x09;
    pub const ARITH_LTS:   u32 = 0x0A;
    pub const ARITH_GTS:   u32 = 0x0B;
    pub const ARITH_LTU:   u32 = 0x0C;
    pub const ARITH_GTU:   u32 = 0x0D;
    pub const ARITH_EQ:    u32 = 0x0E;
    pub const ARITH_NE:    u32 = 0x0F;
}

#[derive(Debug)]
pub enum Instruction {
    R {
        op: u32,
        rs: u8,
        rt: u8,
        rd: u8,
        funct: u32,
    },
    I {
        op: u32,
        rs: u8,
        rt: u8,
        imm: u32,
    },
    J {
        op: u32,
        addr: u32,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            &Instruction::R {
                op,
                rs,
                rt,
                rd,
                funct,
            } => {
                if op != OP_ARITH
                    || !matches!(rs, 0..32)
                    || !matches!(rt, 0..32)
                    || !matches!(rd, 0..32)
                {
                    write!(f, "<invalid>")
                } else {
                    let name = match funct {
                        ARITH_NOT => return write!(f, "not ${rd}, ${rs}, ${rt}"),
                        ARITH_ADD => "add",
                        ARITH_SUB => "sub",
                        ARITH_AND => "and",
                        ARITH_OR => "or",
                        ARITH_XOR => "xor",
                        ARITH_LTS => "lts",
                        ARITH_GTS => "gts",
                        ARITH_LTU => "ltu",
                        ARITH_GTU => "gtu",
                        ARITH_EQ => "eq",
                        ARITH_NE => "ne",

                        _ => return write!(f, "<invalid>"),
                    };

                    write!(f, "{name} ${rd}, ${rs}, ${rt}")
                }
            }
            &Instruction::I { op, rs, rt, imm } => {
                if !matches!(rs, 0..32) || !matches!(rt, 0..32) {
                    write!(f, "<invalid>")
                } else {
                    match op {
                        OP_SHI => write!(f, "shi ${rt}, {imm}"),
                        OP_SLO => write!(f, "slo ${rt}, {imm}"),
                        OP_LOAD => write!(f, "load ${rt}, ${rs}"),
                        OP_STORE => write!(f, "store ${rt}, ${rs}"),
                        OP_JUMP_REG => write!(f, "jmp ${rs}"),
                        OP_BRANCH => write!(f, "br ${rs}, 0x{imm:X}"),

                        _ => write!(f, "<invalid>"),
                    }
                }
            }
            &Instruction::J { op, addr } => {
                if addr > 0x3FFFFFF {
                    write!(f, "<invalid>")
                } else {
                    match op {
                        OP_JUMP_IMM => write!(f, "jmp 0x{addr:X}"),
                        OP_HALT => write!(f, "halt"),
                        OP_NOP => write!(f, "nop"),

                        _ => write!(f, "<invalid>"),
                    }
                }
            }
        }
    }
}

pub fn parse_instruction(word: u32) -> Instruction {
    let op = word >> 26;

    match op {
        0 => {
            let rs = ((word >> 21) & 0x1F) as u8;
            let rt = ((word >> 16) & 0x1F) as u8;
            let rd = ((word >> 11) & 0x1F) as u8;
            let funct = word & 0x3F;

            Instruction::R {
                op,
                rs,
                rt,
                rd,
                funct,
            }
        }
        1..=6 => {
            let rs = ((word >> 21) & 0x1F) as u8;
            let rt = ((word >> 16) & 0x1F) as u8;
            let imm = word & 0xFFFF;

            Instruction::I { op, rs, rt, imm }
        }
        _ => {
            let addr = word & 0x3FFFFFF;

            Instruction::J { op, addr }
        }
    }
}
