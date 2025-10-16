use lalrpop_util::lalrpop_mod;

use crate::lexer::Token;

pub mod ast;

lalrpop_mod!(grammar, "/parser/grammar.rs");

/// Parses the given [Token] stream, producing an [ast::Program] is successful,
/// otherwise returns a [ParseError].
pub fn parse<I>(tokens: I) -> Result<ast::Program, ParseError>
where
    I: IntoIterator<Item = (usize, Token, usize)>,
{
    let parser = grammar::ProgramParser::new();
    parser.parse(tokens.into_iter().map(Ok))
}

pub type ParseError = lalrpop_util::ParseError<usize, Token, &'static str>;
