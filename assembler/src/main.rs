use crate::lexer::Lexer;

mod lexer;
mod strings;

fn main() {
    let input = include_str!("../examples/test.S");

    for token in Lexer::new(input) {
        println!("{token:?}");
    }
}
