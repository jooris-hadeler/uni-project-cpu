use crate::{lexer::Span, strings::StringId};

#[derive(Debug, PartialEq, Eq)]
pub struct Module {
    pub name: StringId,
    pub content: Vec<Node>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Instruction(InstructionNode),
    Label(LabelNode),
    Macro(MacroNode),
    Expansion(ExpansionNode),

    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExpansionNode {
    pub invaktion_name: StringId,
    pub invokation_span: Span,
    pub content: Vec<Node>,
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
        op_span: Span,
        expr: Box<Expression>,
    },

    Binary {
        op: BinaryOp,
        op_span: Span,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    Number {
        value: u32,
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

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Unary { op, op_span, expr } => op_span.join(expr.span()),
            Expression::Binary {
                op,
                op_span,
                left,
                right,
            } => left.span().join(right.span()),
            Expression::Number { value, span } => span,
            Expression::Identifier { name, span } => span,
            Expression::Register { id, span } => span,
        }
    }
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
