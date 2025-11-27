use crate::parser::Parser;

mod expander;
mod lexer;
mod parser;
mod strings;

fn main() {
    let input = include_str!("../examples/basic_macro.S");

    println!("{:#?}", Parser::new(input).parse());
}
