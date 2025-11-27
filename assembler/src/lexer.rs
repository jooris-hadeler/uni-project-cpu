use std::str::Chars;

use multipeek::{IteratorExt, MultiPeek};

use crate::strings::{StringId, intern};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn join(&self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Error,

    Newline,

    Identifier,
    Register,
    Number,

    KwMacro,
    KwInclude,

    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,

    Plus,
    Minus,
    Asterisk,
    Slash,

    Ampersand,
    Pipe,
    Caret,
    ShiftLeft,
    ShiftRight,

    Comma,
    Colon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub text: Option<StringId>,
}

pub struct Lexer<'input> {
    input: &'input str,
    stream: MultiPeek<Chars<'input>>,
    position: usize,
    has_emitted_newline: bool,
}

impl<'input> Lexer<'input> {
    /// Create a new [Lexer] from the given input.
    pub fn new(input: &'input str) -> Self {
        let stream = input.chars().multipeek();

        Self {
            input,
            stream,
            position: 0,
            has_emitted_newline: false,
        }
    }

    /// Peek at the n-th next character in the input stream.
    fn peek(&mut self, n: usize) -> Option<char> {
        self.stream.peek_nth(n).copied()
    }

    /// Consumes the next character in the input stream.
    fn bump(&mut self) -> Option<char> {
        let ch = self.stream.next()?;
        self.position += ch.len_utf8();
        Some(ch)
    }

    /// This method skips whitespace and comments
    fn skip_whitespace(&mut self) -> Option<Token> {
        let start = self.position;

        loop {
            let iteration_start = self.position;

            // skip whitespace
            while self.peek(0).is_some_and(char::is_whitespace) {
                self.bump();
            }

            // skip line comments
            if self.peek(0) == Some('/') && self.peek(1) == Some('/') {
                while self.peek(0).is_some_and(|ch| ch != '\n') {
                    self.bump();
                }
            }

            if iteration_start == self.position {
                break;
            }
        }

        if !self.input[start..self.position].contains('\n') {
            return None;
        }

        let span = Span {
            start,
            end: self.position,
        };

        Some(Token {
            kind: TokenKind::Newline,
            span,
            text: None,
        })
    }

    /// This method handles identifier tokens.
    fn handle_identifier(&mut self) -> Option<Token> {
        let start = self.position;

        while self
            .peek(0)
            .is_some_and(|ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
        {
            self.bump();
        }

        let (kind, text) = match &self.input[start..self.position] {
            "macro" => (TokenKind::KwMacro, None),
            "include" => (TokenKind::KwInclude, None),

            text => (TokenKind::Identifier, Some(intern(text))),
        };

        let span = Span {
            start,
            end: self.position,
        };

        Some(Token { kind, span, text })
    }

    /// This method handles number tokens.
    fn handle_number(&mut self) -> Option<Token> {
        let start = self.position;

        while self.peek(0).is_some_and(|ch| ch.is_ascii_digit()) {
            self.bump();
        }

        let text = Some(intern(&self.input[start..self.position]));

        let span = Span {
            start,
            end: self.position,
        };

        Some(Token {
            kind: TokenKind::Number,
            span,
            text,
        })
    }

    /// This method handles register tokens.
    fn handle_register(&mut self) -> Option<Token> {
        let start = self.position;

        // skip $ character
        self.bump();

        while self.peek(0).is_some_and(|ch| ch.is_ascii_digit()) {
            self.bump();
        }

        let text = Some(intern(&self.input[start + 1..self.position]));

        let span = Span {
            start,
            end: self.position,
        };

        Some(Token {
            kind: TokenKind::Register,
            span,
            text,
        })
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tok) = self.skip_whitespace() {
            return Some(tok);
        }

        macro_rules! token {
            // Base case: the `_ => Default` branch
            (_ => $default:ident $(,)?) => {{
                self.bump();
                TokenKind::$default
            }};

            // Recursive case: 'x' => Variant, ...
            ($ch:literal => $kind:ident, $($rest:tt)*) => {
                if self.peek(1).is_some_and(|ch| ch == $ch) {
                    self.bump();
                    self.bump();
                    TokenKind::$kind
                } else {
                    token! { $($rest)* }
                }
            };
        }

        let start = self.position;

        let Some(peek_ch) = self.peek(0) else {
            if !self.has_emitted_newline {
                self.has_emitted_newline = true;

                let span = Span { start, end: start };

                return Some(Token {
                    kind: TokenKind::Newline,
                    span,
                    text: None,
                });
            }

            return None;
        };

        let kind = match peek_ch {
            'a'..='z' | 'A'..='Z' | '_' => return self.handle_identifier(),
            '0'..='9' => return self.handle_number(),
            '$' => return self.handle_register(),

            '+' => token!(_ => Plus),
            '-' => token!(_ => Minus),
            '*' => token!(_ => Asterisk),
            '/' => token!(_ => Slash),
            '&' => token!(_ => Ampersand),
            '|' => token!(_ => Pipe),
            '^' => token!(_ => Caret),

            '>' => token! {
                '>' => ShiftRight,
                _ => Error,
            },
            '<' => token! {
                '<' => ShiftLeft,
                _ => Error,
            },

            '{' => token!(_ => LeftBrace),
            '}' => token!(_ => RightBrace),
            '(' => token!(_ => LeftParen),
            ')' => token!(_ => RightParen),

            ',' => token!(_ => Comma),
            ':' => token!(_ => Colon),

            _ => token!(_ => Error),
        };

        let text = None;
        let span = Span {
            start,
            end: self.position,
        };

        Some(Token { kind, span, text })
    }
}
