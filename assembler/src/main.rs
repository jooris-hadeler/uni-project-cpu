mod asm;
mod ast;

use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    process::exit,
};

use clap::Parser;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

#[derive(Debug, clap::Parser)]
struct CLI {
    /// Assembly file to process.
    input: PathBuf,

    /// Output file to write bytecode to.
    #[arg(short, long, default_value = "a.out")]
    output: PathBuf,

    /// Insert 4 nop instructions after every real instruction.
    #[arg(short, long)]
    insert_nops: bool,
}

fn main() {
    let cli = CLI::parse();

    let input = match fs::read_to_string(&cli.input) {
        Ok(input) => input,
        Err(err) => {
            eprintln!(
                "Error: failed to read input file `{}` ({:?})",
                cli.input.display(),
                err.kind()
            );
            exit(-1);
        }
    };

    let parser = grammar::ProgramParser::new();

    let program = match parser.parse(&input) {
        Ok(program) => program,
        Err(err) => {
            eprint!("Error:");

            match err {
                lalrpop_util::ParseError::InvalidToken { location } => {
                    eprintln!("encountered invalid token at {location}")
                }
                lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
                    eprint!("unexpected end of file at {location}, expected one of: ");

                    let mut first = true;
                    for item in expected {
                        if !first {
                            eprint!(", ");
                        } else {
                            first = false;
                        }

                        eprint!("`{item}`");
                    }

                    eprintln!("instead.");
                }
                lalrpop_util::ParseError::UnrecognizedToken {
                    token: (s, tok, e),
                    expected,
                } => {
                    eprint!("unexpected token at {s}-{e}, expected one of: ");

                    let mut first = true;
                    for item in expected {
                        if !first {
                            eprint!(", ");
                        } else {
                            first = false;
                        }

                        eprint!("`{item}`");
                    }

                    eprintln!("found `{tok}` instead.");
                }
                lalrpop_util::ParseError::ExtraToken { token: (s, tok, e) } => {
                    eprintln!("unexpected extra token `{tok}` at {s}-{e}.");
                }
                lalrpop_util::ParseError::User { error } => eprintln!("{error}"),
            }

            exit(-1);
        }
    };

    let code = match asm::assemble(&program, cli.insert_nops) {
        Ok(code) => code,
        Err(e) => {
            eprint!("Error assembling:");

            match e {
                asm::AssemblerError::UndefinedLabel(label) => {
                    eprintln!("undefined label `{label}` referenced")
                }
                asm::AssemblerError::AddressOutOfBounds26(addr) => {
                    eprintln!("address 0x{addr:X} does not fit in 26 bits")
                }
                asm::AssemblerError::AddressOutOfBounds16(addr) => {
                    eprintln!("address 0x{addr:X} does not fit in 16 bits")
                }
            }
            exit(-1);
        }
    };

    match write_to_file(&cli.output, &code) {
        Ok(_) => eprintln!("Done! Assembled successfully."),
        Err(err) => {
            eprintln!(
                "Error: failed to write output file `{}` ({:?})",
                cli.input.display(),
                err.kind()
            );
            exit(-1);
        }
    }
}

fn write_to_file(path: &PathBuf, buffer: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(buffer)?;
    file.flush()
}
