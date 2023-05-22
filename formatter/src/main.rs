use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    input_file: PathBuf,
    output_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input_file = File::open(args.input_file)?;
    let reader = BufReader::new(input_file);
    let output_file = File::create(args.output_file)?;
    let mut writer = BufWriter::new(output_file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        writer.write(format!("when \"{:06b}\" => \n", index).as_bytes())?;
        writer.write(format!("\tIMOut <= \"{}\"\n", line).as_bytes())?;
    }
    Ok(())
}
