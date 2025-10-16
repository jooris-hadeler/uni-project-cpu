use std::{fmt::Display, num::ParseIntError};

use logos::Logos;

use crate::util::{StringId, intern};

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
#[logos(skip r"[ \r\t]+", error = LexError)]
pub enum Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", handle_identifier)]
    Identifier(StringId),

    #[regex(r"\$[a-z0-9]+", handle_register)]
    Register(Register),

    #[regex(r"-?0x[0-9A-Fa-f]+", handle_number::<16>)]
    #[regex(r"-?0o[0-7]+", handle_number::<8>)]
    #[regex(r"-?0b[01]+", handle_number::<2>)]
    #[regex(r"-?[0-9]+", handle_number::<10>)]
    Number(i64),

    #[regex(r"%[0-9]+", handle_parameter)]
    Parameter(usize),

    #[regex("\"[^\"]*\"", handle_string)]
    String(StringId),

    #[token("define")]
    KwDefine,
    #[token("macro")]
    KwMacro,
    #[token("end")]
    KwEnd,
    #[token("include")]
    KwInclude,

    #[token(":")]
    Colon,
    #[token(",")]
    Comma,

    #[regex(r"--.*", handle_newline_or_comment)]
    #[regex(r"\n+", handle_newline_or_comment)]
    Newline,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(_) => write!(f, "Identifier"),
            Token::Register(_) => write!(f, "Register"),
            Token::Number(_) => write!(f, "Number"),
            Token::Parameter(_) => write!(f, "Parameter"),
            Token::String(_) => write!(f, "String"),
            Token::KwDefine => write!(f, "`define`"),
            Token::KwMacro => write!(f, "`macro`"),
            Token::KwEnd => write!(f, "`end`"),
            Token::KwInclude => write!(f, "`include`"),
            Token::Colon => write!(f, "`:`"),
            Token::Comma => write!(f, "`,`"),
            Token::Newline => write!(f, "Newline"),
        }
    }
}

/// Callback that interns identifier tokens, for easier use.
fn handle_identifier<'inpt>(lex: &mut logos::Lexer<'inpt, Token>) -> Result<StringId, LexError> {
    Ok(intern(lex.slice()))
}

/// Callback that parses register tokens.
fn handle_register<'inpt>(lex: &mut logos::Lexer<'inpt, Token>) -> Result<Register, LexError> {
    let text = &lex.slice()[1..];

    Ok(match text {
        "0" => Register::R0,
        "1" => Register::R1,
        "2" => Register::R2,
        "3" => Register::R3,
        "4" => Register::R4,
        "5" => Register::R5,
        "6" => Register::R6,
        "7" => Register::R7,
        "8" => Register::R8,
        "9" => Register::R9,
        "10" => Register::R10,
        "11" => Register::R11,
        "12" => Register::R12,
        "13" => Register::R13,
        "14" => Register::R14,
        "15" => Register::R15,
        "16" => Register::R16,
        "17" => Register::R17,
        "18" => Register::R18,
        "19" => Register::R19,
        "20" => Register::R20,
        "21" => Register::R21,
        "22" => Register::R22,
        "23" => Register::R23,
        "24" => Register::R24,
        "25" => Register::R25,
        "26" => Register::R26,
        "27" => Register::R27,
        "28" => Register::R28,
        "bp" => Register::Rbp,
        "sp" => Register::Rsp,
        "ip" => Register::Rip,

        _ => return Err(LexError::InvalidRegister),
    })
}

/// Callback that parses number tokens.
fn handle_number<'inpt, const BASE: u32>(
    lex: &mut logos::Lexer<'inpt, Token>,
) -> Result<i64, LexError> {
    let text = lex.slice();

    let (negative, text) = text
        .strip_prefix('-')
        .map(|rest| (true, rest))
        .unwrap_or((false, text));

    let text = match BASE {
        16 | 8 | 2 => &text[2..],
        10 => text,
        _ => unreachable!(),
    };

    let value = i64::from_str_radix(text, BASE)?;

    Ok(match negative {
        true => -value,
        false => value,
    })
}

/// Callback that parses parameter tokens.
fn handle_parameter<'inpt>(lex: &mut logos::Lexer<'inpt, Token>) -> Result<usize, LexError> {
    let text = &lex.slice()[1..];

    text.parse().map_err(LexError::InvalidArgument)
}

/// Callback that interns the contents of a string.
fn handle_string<'inpt>(lex: &mut logos::Lexer<'inpt, Token>) -> Result<StringId, LexError> {
    let text = lex
        .slice()
        .strip_prefix("\"")
        .and_then(|text| text.strip_suffix("\""))
        .unwrap();

    Ok(intern(text))
}

/// Callback that merges consecutive newlines, possibly separated by comments.
fn handle_newline_or_comment<'inpt>(lex: &mut logos::Lexer<'inpt, Token>) -> Result<(), LexError> {
    loop {
        let remainder = lex.remainder();

        // Skip whitespace before next comment or newline
        let ws_len = remainder
            .chars()
            .take_while(|c| matches!(c, ' ' | '\t' | '\r'))
            .count();

        if ws_len > 0 {
            lex.bump(ws_len);
        }

        let remainder = lex.remainder();

        if remainder.starts_with("--") {
            // Skip the comment line entirely
            if let Some(pos) = remainder.find('\n') {
                lex.bump(pos + 1);
            } else {
                // EOF after comment
                lex.bump(remainder.len());
                break;
            }
        } else if remainder.starts_with('\n') {
            // Skip additional newline
            lex.bump(1);
        } else {
            // No more newlines/comments
            break;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum LexError {
    InvalidNumber(ParseIntError),
    InvalidArgument(ParseIntError),
    InvalidRegister,

    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidNumber(value)
    }
}

#[repr(u32)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {  
    R0 = 0,
    R1,  R2,  R3,  R4,  R5,  R6,  R7,  R8,  R9,
    R10, R11, R12, R13, R14, R15, R16, R17, R18,
    R19, R20, R21, R22, R23, R24, R25, R26, R27, 
    R28, 
    Rbp, Rsp, Rip,
}

/// Return type of the [lex] function.
type LexResult = Result<Vec<(usize, Token, usize)>, Vec<(usize, LexError, usize)>>;

/// Lexes the given input, producing a [Vec] of [Token] if successful,
/// otherwise returns a [Vec] of [LexError].
pub fn lex(input: &str) -> LexResult {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for (result, span) in Token::lexer(input).spanned() {
        match result {
            Ok(tok) => tokens.push((span.start, tok, span.end)),
            Err(err) => errors.push((span.start, err, span.end)),
        }
    }

    // Ensure we have a Newline Token at the end.
    if let Some((_, tok, e)) = tokens.last()
        && tok != &Token::Newline
    {
        tokens.push((*e + 1, Token::Newline, *e + 2));
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}
