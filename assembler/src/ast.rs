#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Instruction(Instruction),
    Label(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Arith {
        op: ArithOp,
        dst: u8,
        src1: u8,
        src2: u8,
    },
    Halt,
    Nop,
    Shi {
        dst: u8,
        imm: u16,
    },
    Slo {
        dst: u8,
        imm: u16,
    },
    JumpLabel {
        label: String,
    },
    JumpRegister {
        target: u8,
    },
    Branch {
        cond: u8,
        label: String,
    },
    Load {
        dst: u8,
        src: u8,
    },
    Store {
        dst: u8,
        src: u8,
    },

    // === PSEUDO INSTRUCTIONS ===
    Mov {
        dst: u8,
        imm: u32,
    },
    Copy {
        dst: u8,
        src: u8,
    },
    Push {
        src: u8,
    },
    Pop {
        dst: u8,
    },
    Call {
        label: String,
    },
    Ret,

    // === HELPER INSTRUCTIONS ===
    PushPc {
        offset: u32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Not,
    Lts,
    Ltu,
    Gts,
    Gtu,
    Eq,
    Ne,
}
