use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::{Result, anyhow};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author = "adudenamedruby", version, about)]
/// Rust version of uniq
struct Args {
    /// Input file
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Shows count
    #[arg(short, long)]
    count: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    let mut file = open(&args.in_file).map_err(|e| anyhow!("{}: {e}", args.in_file))?;
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        print!("{line}");
        line.clear();
    }

    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
