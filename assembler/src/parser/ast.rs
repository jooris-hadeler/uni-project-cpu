use crate::{lexer::Register, util::StringId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub content: Vec<Content>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    Instruction(Instruction),
    Label(Label),
    Macro(Macro),
    Include(Include),
    Expanded(Expanded),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expanded {
    pub invokation_span: (usize, usize),
    pub content: Vec<Content>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Include {
    pub path: StringId,
    pub path_span: (usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Macro {
    pub name: StringId,
    pub name_span: (usize, usize),
    pub num_args: i64,
    pub replacement: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub name: StringId,
    pub name_span: (usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub mnemonic: StringId,
    pub mnemonic_span: (usize, usize),
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Argument {
    pub kind: ArgumentKind,
    pub span: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentKind {
    Identifier(StringId),
    Register(Register),
    Parameter(usize),
    Number(i64),
}
