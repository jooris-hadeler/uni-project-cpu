use std::{fs, path::PathBuf, process::exit};

use clap::Parser;
use cpu::Cpu;

pub mod cpu;
pub mod isa;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Binary file to run.
    pub file: PathBuf,

    /// Print the loaded program and quit.
    #[arg(short, long)]
    pub print_assembly: bool,

    /// Single step through the program.
    #[arg(short, long)]
    pub single_step: bool,

    /// Verbose output.
    #[arg(short, long)]
    pub verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let Ok(rom) = fs::read(&cli.file) else {
        eprintln!("Error: failed to load ROM image.");
        exit(-1);
    };

    let rom: Vec<u32> = rom
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
        .collect();

    let mut cpu = Cpu::new(rom);

    if cli.print_assembly {
        cpu.print_assembly();
        exit(0);
    }

    cpu.run(cli.single_step, cli.verbose);
    cpu.dump();
}
