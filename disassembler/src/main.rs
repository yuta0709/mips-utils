use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    input_file: PathBuf,
    output_file: PathBuf,
}

fn to_twos_complement(n: u32, bits: u32) -> i32 {
    let mask = 1 << (bits - 1);
    let value = n & (mask - 1);
    if n & mask != 0 {
        value as i32 - mask as i32
    } else {
        value as i32
    }
}

fn register_name(r: u32) -> Option<String> {
    match r {
        0 => Some("$zero".to_string()),
        1 => Some("$at".to_string()),
        2 | 3 => Some(format!("$v{}", r - 2)),
        4..=7 => Some(format!("$a{}", r - 4)),
        8..=15 => Some(format!("$t{}", r - 8)),
        16..=23 => Some(format!("$s{}", r - 16)),
        24..=25 => Some(format!("$t{}", r - 16)),
        26..=27 => Some(format!("$k{}", r - 26)),
        28 => Some("$gp".to_string()),
        29 => Some("$sp".to_string()),
        30 => Some("$fp".to_string()),
        31 => Some("$ra".to_string()),
        _ => None,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input_file = File::open(args.input_file)?;
    let output_file = File::create(args.output_file)?;
    let mut writer = BufWriter::new(output_file);

    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let line = line?;
        let instruction = u32::from_str_radix(line.as_str(), 2)?;

        let opcode = instruction >> 26;
        let rs = (instruction >> 21) & 0x1F;
        let rs = register_name(rs).unwrap();
        let rt = (instruction >> 16) & 0x1F;
        let rt = register_name(rt).unwrap();
        let rd = (instruction >> 11) & 0x1F;
        let rd = register_name(rd).unwrap();

        let immediate = instruction & 0xFFFF;
        let immediate = to_twos_complement(immediate, 16);
        let addr = instruction & 0x3FFFFFF;

        match opcode {
            0 => writer.write(format!("add {}, {}, {}\n", rd, rs, rt).as_bytes())?,
            1 => writer.write(format!("addi {}, {}, {}\n", rt, rs, immediate).as_bytes())?,
            2 => writer.write(format!("lw {}, {}({})\n", rt, immediate, rs).as_bytes())?,
            3 => writer.write(format!("sw {}, {}({})\n", rt, immediate, rs).as_bytes())?,
            4 => writer.write(format!("beq {}, {}, {}\n", rs, rt, immediate).as_bytes())?,
            5 => writer.write(format!("j {}\n", addr).as_bytes())?,
            6 => writer.write(format!("jal {}\n", addr).as_bytes())?,
            7 => writer.write(format!("jr {}\n", rs).as_bytes())?,
            _ => return Err(anyhow!("unsupported instruction")),
        };
    }
    Ok(())
}
