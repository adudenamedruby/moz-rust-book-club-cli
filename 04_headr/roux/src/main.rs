use std::{
    fs::{File, metadata},
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about, long_about=None)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short = 'n',
        long,
        default_value = "10",
        conflicts_with = "bytes",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// Number of bytes
    #[arg(short = 'c', long, value_parser = clap::value_parser!(u64).range(1..))]
    bytes: Option<u64>,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    for filename in &args.files {
        match open(filename) {
            Err(e) => eprintln!("{filename}: {e}\n"),
            Ok(mut file) => match metadata(filename) {
                Err(e) => eprintln!("{filename}: {e}"),
                Ok(metadata) => {
                    if metadata.len() != 0 {
                        if args.files.len() > 1 {
                            print!("==> {filename} <==");
                        }

                        if let Some(byte_num) = args.bytes {
                            let mut buf = vec![0; byte_num as usize];
                            file.read_exact(&mut buf)?;
                            let byte_str = String::from_utf8_lossy(&buf);
                            print!("{}", byte_str);
                        } else {
                            let mut buf = String::new();
                            for _ in 0..args.lines {
                                buf.clear();
                                file.read_line(&mut buf)?;
                                print!("{buf}");
                            }
                        }
                    }
                }
            },
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
