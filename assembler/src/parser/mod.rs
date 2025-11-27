use std::{collections::HashMap, iter::once, num::ParseIntError};

use lazy_static::lazy_static;
use multipeek::{IteratorExt, MultiPeek};

use crate::{
    lexer::{Lexer, Span, Token, TokenKind},
    parser::ast::*,
    strings::resolve,
};

pub mod ast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    UnexpectedToken {
        span: Span,
        expected: Vec<TokenKind>,
        found: TokenKind,
    },

    UnexpectedEndOfFile {
        expected: Vec<TokenKind>,
    },

    InvalidNumberLiteral {
        error: ParseIntError,
        span: Span,
    },

    InvalidArgumentKind {
        span: Span,
    },

    InvalidRegister {
        span: Span,
    },
}

pub struct Parser<'input> {
    stream: MultiPeek<Lexer<'input>>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        let stream = Lexer::new(input).multipeek();

        Self { stream }
    }

    fn bump(&mut self) -> Option<Token> {
        self.stream.next()
    }

    fn peek(&mut self, n: usize) -> Option<&Token> {
        self.stream.peek_nth(n)
    }

    fn is_peek(&mut self, n: usize, kind: TokenKind) -> bool {
        self.peek(n).is_some_and(|tok| tok.kind == kind)
    }

    fn expect<'a, I: IntoIterator<Item = &'a TokenKind>>(
        &mut self,
        expected: I,
    ) -> Result<Token, ParserError> {
        let expected = expected.into_iter().copied().collect();

        let Some(peek_token) = self.peek(0) else {
            return Err(ParserError::UnexpectedEndOfFile { expected });
        };

        if !expected.contains(&peek_token.kind) {
            return Err(ParserError::UnexpectedToken {
                span: peek_token.span,
                expected,
                found: peek_token.kind,
            });
        }

        Ok(self.bump().unwrap())
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, ParserError> {
        // skip newline, at the beginning of a file
        if self.is_peek(0, TokenKind::Newline) {
            self.bump();
        }

        let mut nodes = Vec::new();

        while self.peek(0).is_some() {
            nodes.push(self.parse_node()?);
        }

        Ok(nodes)
    }

    fn parse_node(&mut self) -> Result<Node, ParserError> {
        assert!(self.peek(0).is_some());

        let peek_token = self.peek(0).unwrap();

        match peek_token.kind {
            TokenKind::KwMacro => self.parse_macro().map(Node::Macro),
            TokenKind::Identifier => self.parse_label_or_instruction(),

            _ => Err(ParserError::UnexpectedToken {
                span: peek_token.span,
                expected: vec![TokenKind::KwMacro, TokenKind::Identifier],
                found: peek_token.kind,
            }),
        }
    }

    fn parse_macro(&mut self) -> Result<MacroNode, ParserError> {
        self.expect(&[TokenKind::KwMacro])?;

        let (name, name_span) = {
            let Token { span, text, .. } = self.expect(&[TokenKind::Identifier])?;
            (text.unwrap(), span)
        };

        let mut args = Vec::new();
        self.expect(&[TokenKind::LeftParen])?;

        while self
            .peek(0)
            .is_some_and(|tok| tok.kind != TokenKind::RightParen)
        {
            if !args.is_empty() {
                self.expect(&[TokenKind::Comma])?;
            }

            let (name, name_span) = {
                let Token { span, text, .. } = self.expect(&[TokenKind::Identifier])?;
                (text.unwrap(), span)
            };

            self.expect(&[TokenKind::Colon])?;

            let (kind, kind_span) = {
                let Token { span, text, .. } = self.expect(&[TokenKind::Identifier])?;

                let kind = match resolve(text.unwrap()).as_str() {
                    "reg" => ArgumentKind::Register,
                    "num" => ArgumentKind::Number,

                    _ => return Err(ParserError::InvalidArgumentKind { span }),
                };

                (kind, span)
            };

            args.push(Argument {
                name,
                name_span,
                kind,
                kind_span,
            });
        }

        self.expect(&[TokenKind::RightParen])?;

        let body = self.parse_block()?;

        if self.is_peek(0, TokenKind::Newline) {
            self.bump();
        }

        Ok(MacroNode {
            name,
            name_span,
            args,
            body,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Node>, ParserError> {
        let mut nodes = Vec::new();

        if self.is_peek(0, TokenKind::Newline) {
            self.bump();
        }

        self.expect(&[TokenKind::LeftBrace])?;

        if self.is_peek(0, TokenKind::Newline) {
            self.bump();
        }

        while self
            .peek(0)
            .is_some_and(|tok| tok.kind != TokenKind::RightBrace)
        {
            nodes.push(self.parse_label_or_instruction()?);
        }

        self.expect(&[TokenKind::RightBrace])?;

        Ok(nodes)
    }

    fn parse_label_or_instruction(&mut self) -> Result<Node, ParserError> {
        let ident = self.expect(&[TokenKind::Identifier])?;

        // if the next token is a `:` we have a label
        if self.is_peek(0, TokenKind::Colon) {
            self.bump();

            self.expect(&[TokenKind::Newline])?;

            return Ok(Node::Label(LabelNode {
                name: ident.text.unwrap(),
                name_span: ident.span,
            }));
        }

        let mut arguments = Vec::new();

        while self
            .peek(0)
            .is_some_and(|tok| tok.kind != TokenKind::Newline)
        {
            if !arguments.is_empty() {
                self.expect(&[TokenKind::Comma])?;
            }

            arguments.push(self.parse_expression()?);
        }

        self.expect(&[TokenKind::Newline])?;

        Ok(Node::Instruction(InstructionNode {
            mnemonic: ident.text.unwrap(),
            mnemonic_span: ident.span,
            arguments,
        }))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_expr_impl(0)
    }

    fn parse_expr_impl(&mut self, binding_power: u8) -> Result<Expression, ParserError> {
        let expected = once(TokenKind::Number)
            .chain(once(TokenKind::Register))
            .chain(once(TokenKind::Identifier))
            .chain(once(TokenKind::LeftParen))
            .chain(PREFIX_OPS.keys().copied())
            .collect();

        let Some(first_token) = self.bump() else {
            return Err(ParserError::UnexpectedEndOfFile { expected });
        };

        let mut lhs = match first_token.kind {
            TokenKind::Number => {
                let value = resolve(first_token.text.unwrap())
                    .parse::<u32>()
                    .map_err(|error| ParserError::InvalidNumberLiteral {
                        error,
                        span: first_token.span,
                    })?;

                Expression::Number {
                    value,
                    span: first_token.span,
                }
            }
            TokenKind::LeftParen => {
                let expr = self.parse_expr_impl(0)?;

                self.expect(&[TokenKind::RightParen])?;

                expr
            }
            TokenKind::Identifier => Expression::Identifier {
                name: first_token.text.unwrap(),
                span: first_token.span,
            },
            TokenKind::Register => {
                let id = resolve(first_token.text.unwrap())
                    .parse::<u32>()
                    .map_err(|error| ParserError::InvalidNumberLiteral {
                        error,
                        span: first_token.span,
                    })?;

                if id >= 32 {
                    return Err(ParserError::InvalidRegister {
                        span: first_token.span,
                    });
                }

                Expression::Register {
                    id,
                    span: first_token.span,
                }
            }
            tok if PREFIX_OPS.contains_key(&tok) => {
                let (op, bp) = PREFIX_OPS.get(&tok).copied().unwrap();

                let expr = Box::new(self.parse_expr_impl(bp)?);

                Expression::Unary {
                    op,
                    op_span: first_token.span,
                    expr,
                }
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    span: first_token.span,
                    expected,
                    found: first_token.kind,
                });
            }
        };

        loop {
            let Some(peek_token) = self.peek(0) else {
                return Err(ParserError::UnexpectedEndOfFile {
                    expected: once(TokenKind::Number)
                        .chain(INFIX_OPS.keys().copied())
                        .collect(),
                });
            };

            let op_span = peek_token.span;

            let Some((op, left_binding_power, right_binding_power)) =
                INFIX_OPS.get(&peek_token.kind).copied()
            else {
                break;
            };

            if left_binding_power < binding_power {
                break;
            }

            self.bump();

            let rhs = self.parse_expr_impl(right_binding_power)?;

            lhs = Expression::Binary {
                op,
                op_span,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }
}

lazy_static! {
    static ref PREFIX_OPS: HashMap<TokenKind, (UnaryOp, u8)> = {
        let mut map = HashMap::new();

        map.insert(TokenKind::Minus, (UnaryOp::Negate, 90));

        map
    };
    static ref INFIX_OPS: HashMap<TokenKind, (BinaryOp, u8, u8)> = {
        let mut map = HashMap::new();

        map.insert(TokenKind::Asterisk, (BinaryOp::Multiply, 80, 81));
        map.insert(TokenKind::Slash, (BinaryOp::Divide, 80, 81));

        map.insert(TokenKind::Plus, (BinaryOp::Add, 70, 71));
        map.insert(TokenKind::Minus, (BinaryOp::Subtract, 70, 71));

        map.insert(TokenKind::ShiftLeft, (BinaryOp::ShiftLeft, 60, 61));
        map.insert(TokenKind::ShiftRight, (BinaryOp::ShiftRight, 60, 61));

        map.insert(TokenKind::Ampersand, (BinaryOp::And, 50, 51));

        map.insert(TokenKind::Caret, (BinaryOp::Xor, 40, 41));

        map.insert(TokenKind::Pipe, (BinaryOp::Or, 30, 31));

        map
    };
}
