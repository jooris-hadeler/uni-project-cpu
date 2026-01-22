#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Real(RealInstruction),
    Pseudo(PseudoInstruction),
    Label(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RealInstruction {
    Arith { op: ArithOp, rd: u8, rs: u8, rt: u8 },
    Shi { rs: u8, rt: u8, imm: u16 },
    Slo { rs: u8, rt: u8, imm: u16 },
    Load { rs: u8, rt: u8, imm: i16 },
    Store { rs: u8, rt: u8, imm: i16 },
    Jr { rs: u8 },
    Nop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PseudoInstruction {
    Mov { dst: u8, imm: u32 },
    Copy { dst: u8, src: u8 },

    Br { rs: u8, rt: u8, label: String },
    Jmp { label: String },
    Jar { label: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Sar,
    Not,
    Lts,
    Ltu,
    Gts,
    Gtu,
    Eq,
    Ne,
}
