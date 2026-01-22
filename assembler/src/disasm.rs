use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, clap::Parser)]
struct CLI {
    /// Assembly file to process.
    input: PathBuf,

    /// Should we ignore nops?
    #[arg(short, long)]
    ignore_nops: bool,
}

fn sign_extend_16(x: u16) -> i16 {
    x as i16
}

fn reg(r: u8) -> String {
    format!("${}", r)
}

fn format_signed_hex<T>(val: T) -> String
where
    T: num_traits::Signed + num_traits::ToPrimitive,
{
    if val.is_negative() {
        format!("-0x{:X}", val.abs().to_u64().unwrap())
    } else {
        format!("0x{:X}", val.to_u64().unwrap())
    }
}

fn decode_r(rs: u8, rt: u8, rd: u8, funct: u8) -> String {
    match funct {
        0 => format!("add {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        1 => format!("sub {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        2 => format!("and {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        3 => format!("or {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        4 => format!("xor {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        5 => format!("shl {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        6 => format!("sal {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        7 => format!("shr {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        8 => format!("sar {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        9 => format!("not {}, {}", reg(rd), reg(rs)),
        10 => format!("lts {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        11 => format!("gts {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        12 => format!("ltu {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        13 => format!("gtu {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        14 => format!("eq {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        15 => format!("ne {}, {}, {}", reg(rd), reg(rs), reg(rt)),
        _ => format!("unknown_r funct={}", funct),
    }
}

fn decode_i(opcode: u8, rs: u8, rt: u8, imm: u16) -> String {
    match opcode {
        1 => format!("shi {}, {}, 0x{:X}", reg(rs), reg(rt), imm),
        2 => format!("slo {}, {}, 0x{:X}", reg(rs), reg(rt), imm),
        3 => {
            let simm = sign_extend_16(imm);
            format!("load {}, {}({})", reg(rt), format_signed_hex(simm), reg(rs))
        }
        4 => {
            let simm = sign_extend_16(imm);
            format!(
                "store {}, {}({})",
                reg(rt),
                format_signed_hex(simm),
                reg(rs)
            )
        }
        5 => {
            let simm = sign_extend_16(imm);
            format!("br {}, {}, {}", reg(rs), reg(rt), format_signed_hex(simm))
        }
        6 => format!("jr {}", reg(rs)),
        _ => format!("unknown_i opcode={}", opcode),
    }
}

fn decode_j(opcode: u8, addr: u32) -> String {
    let addr = (0xFC00_0000 | addr) as i32;

    match opcode {
        7 => format!("jmp {}", format_signed_hex(addr)),
        8 => format!("jar {}", format_signed_hex(addr)),
        _ => format!("unknown_j opcode={}", opcode),
    }
}

fn main() -> io::Result<()> {
    let args = CLI::parse();

    let bytes = fs::read(args.input)?;

    if bytes.len() % 4 != 0 {
        eprintln!("File size is not a multiple of 4 bytes");
        std::process::exit(1);
    }

    let mut addr = 0u32;

    for chunk in bytes.chunks(4) {
        let instr = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);

        let opcode = ((instr >> 26) & 0x3F) as u8;

        let disasm = if opcode == 0 {
            let rs = ((instr >> 21) & 0x1F) as u8;
            let rt = ((instr >> 16) & 0x1F) as u8;
            let rd = ((instr >> 11) & 0x1F) as u8;
            let funct = (instr & 0x3F) as u8;

            decode_r(rs, rt, rd, funct)
        } else if opcode <= 6 {
            let rs = ((instr >> 21) & 0x1F) as u8;
            let rt = ((instr >> 16) & 0x1F) as u8;
            let imm = (instr & 0xFFFF) as u16;

            decode_i(opcode, rs, rt, imm)
        } else if opcode <= 8 {
            let addr26 = instr & 0x03FF_FFFF;
            decode_j(opcode, addr26)
        } else if opcode == 63 {
            if args.ignore_nops {
                addr += 1;
                continue;
            }

            "nop".to_string()
        } else {
            format!("unknown opcode={}", opcode)
        };

        println!("0x{:X}: {}", addr, disasm);
        addr += 1;
    }

    Ok(())
}
