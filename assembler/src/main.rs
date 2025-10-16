use std::{fs, path::PathBuf, process::exit};

use clap::Parser;

use crate::{
    lexer::Token,
    util::{intern, resolve},
};

pub mod expand;
pub mod lexer;
pub mod parser;
pub mod util;

#[derive(clap::Parser)]
#[clap(version, about)]
pub struct Cli {
    /// File we want to assemble.
    pub file: PathBuf,

    /// Path of the output file.
    #[arg(short, long, default_value = "a.out")]
    pub output: PathBuf,

    /// Only run the lexer and dump the tokens.
    #[arg(short, long)]
    pub lex: bool,

    /// Only run the lexer and parser then dump the AST.
    #[arg(short, long)]
    pub parse: bool,

    /// Onlz run the lexer, parser and expander then dump the expanded AST.
    #[arg(short, long)]
    pub expand: bool,
}

fn main() {
    let cli = Cli::parse();

    let content = match fs::read_to_string(&cli.file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!(
                "Error: failed to read file {} ({:?})",
                cli.file.display(),
                err.kind()
            );
            exit(1);
        }
    };

    let tokens = match lexer::lex(&content) {
        Ok(tokens) => tokens,
        Err(errors) => {
            eprintln!("Error: an error occured during lexing");

            for (s, err, e) in errors {
                eprintln!("{s}..{e} => {err:?}");
            }

            exit(2);
        }
    };

    // Dump tokens and exit.
    if cli.lex {
        dump_tokens(tokens);
        exit(0);
    }

    let program = match parser::parse(tokens) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Error: an error occured during parsing");
            eprintln!("{err:?}");
            exit(3);
        }
    };

    // Dump the AST.
    if cli.parse {
        println!("{program:#?}");
        exit(0);
    }

    let self_name = intern(cli.file.display().to_string());
    let expanded = match expand::expand(program, self_name) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Error: an error occured during expanding");
            eprintln!("{err:?}");
            exit(4);
        }
    };

    // Dump the expanded AST.
    if cli.expand {
        println!("{expanded:#?}");
        exit(0);
    }
}

fn dump_tokens(tokens: Vec<(usize, Token, usize)>) {
    for (start, token, end) in tokens {
        print!("{start}..{end} => ");

        match token {
            Token::Identifier(id) => println!("Identifier {:?}", resolve(id)),
            Token::Register(reg) => println!("Register {:?}", reg),
            Token::Parameter(idx) => println!("Parameter {}", idx),
            Token::Number(num) => println!("Number {}", num),
            Token::String(id) => println!("String {:?}", resolve(id)),
            Token::KwDefine => println!("KwDefine"),
            Token::KwMacro => println!("KwMacro"),
            Token::KwEnd => println!("KwEnd"),
            Token::KwInclude => println!("KwInclude"),
            Token::Colon => println!("Colon"),
            Token::Comma => println!("Comma"),
            Token::Newline => println!("Newline"),
        }
    }

    exit(0);
}
