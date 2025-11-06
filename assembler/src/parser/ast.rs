use crate::{lexer::Span, strings::StringId};

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Instruction(InstructionNode),
    Label(LabelNode),
    Macro(MacroNode),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LabelNode {
    pub name: StringId,
    pub name_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MacroNode {
    pub name: StringId,
    pub name_span: Span,
    pub args: Vec<Argument>,
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
    pub name: StringId,
    pub name_span: Span,
    pub kind: ArgumentKind,
    pub kind_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArgumentKind {
    Register,
    Number,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstructionNode {
    pub mnemonic: StringId,
    pub mnemonic_span: Span,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },

    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    Number {
        value: u32,
        span: Span,
    },

    Parameter {
        name: StringId,
        span: Span,
    },

    Identifier {
        name: StringId,
        span: Span,
    },

    Register {
        id: u32,
        span: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Negate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,

    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
}
