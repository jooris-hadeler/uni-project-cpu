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

use crate::asm::Assembler;
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

    /// Output VHDL mem file
    #[arg(short, long, default_value_t = false)]
    vhdl_mem: bool,
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

    let code = match Assembler::new(cli.insert_nops).assemble(&program) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(-1);
        }
    };

    let result = if cli.vhdl_mem {
        write_vhdl_to_file(&cli.output, code)
    } else {
        write_to_file(&cli.output, code)
    };

    match result {
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

fn write_vhdl_to_file(path: &PathBuf, buffer: Vec<u32>) -> io::Result<()> {
    let mut file = File::create(path)?;

    writeln!(file, "DEPTH = 1024;")?;
    writeln!(file, "WIDTH = 32;")?;
    writeln!(file, "ADDRESS_RADIX = DEC;")?;
    writeln!(file, "DATA_RADIX = BIN;")?;
    writeln!(file, "CONTENT")?;
    writeln!(file, "BEGIN")?;

    for (index, word) in buffer.iter().enumerate() {
        let index = 4 * index;
        writeln!(file, "{index} : {word:0>32b};")?;
    }

    writeln!(file, "END;")?;

    file.flush()
}

fn write_to_file(path: &PathBuf, buffer: Vec<u32>) -> io::Result<()> {
    let mut file = File::create(path)?;

    let buffer = buffer
        .into_iter()
        .flat_map(u32::to_be_bytes)
        .collect::<Vec<u8>>();

    file.write_all(&buffer)?;
    file.flush()
}
