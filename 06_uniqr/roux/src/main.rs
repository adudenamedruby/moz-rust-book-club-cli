use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
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
    let result = count(&args)?;
    if let Some(outfile) = args.out_file {
        let mut f = File::create(outfile)?;
        f.write_all(result.as_bytes())?;
    } else {
        print!("{result}");
    }

    Ok(())
}

fn count(args: &Args) -> Result<String> {
    let mut file = open(&args.in_file).map_err(|e| anyhow!("{}: {e}", args.in_file))?;
    let mut line = String::new();
    let mut count = 0;
    let mut char_tracker = String::new();
    let mut result = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;

        if line.trim_end() == char_tracker.trim_end() {
            count += 1;
        } else if char_tracker.is_empty() {
            char_tracker = line.clone();
            count = 1;
        } else {
            result.push_str(
                format!(
                    "{}{char_tracker}",
                    if args.count {
                        format!("{:>4} ", count.to_string())
                    } else {
                        "".to_string()
                    },
                )
                .as_str(),
            );

            char_tracker = line.clone();
            count = 1;
        }

        if bytes == 0 {
            return Ok(result);
        }

        line.clear();
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
