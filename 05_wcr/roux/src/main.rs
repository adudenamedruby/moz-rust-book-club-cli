use std::{
    fs::File,
    io::{BufRead, BufReader, stdin},
    ops::AddAssign,
};

use anyhow::{Result, bail};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author = "adudenamedruby", version, about)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Show character count
    // Default values colud be done this way, but the author chose another in the `run` function
    // #[arg(short = 'm', long, conflicts_with = "bytes", default_value_t = true)]
    #[arg(short = 'm', long, conflicts_with = "bytes")]
    chars: bool,
}

#[derive(Debug, PartialEq, Default)]
struct Counts {
    num_lines: usize,
    num_bytes: usize,
    num_chars: usize,
    num_words: usize,
}

impl AddAssign for Counts {
    fn add_assign(&mut self, other: Self) {
        self.num_lines += other.num_lines;
        self.num_words += other.num_words;
        self.num_bytes += other.num_bytes;
        self.num_chars += other.num_chars;
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut file: impl BufRead) -> Result<Counts> {
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut num_words = 0;

    let mut buffer = String::new();

    loop {
        buffer.clear();
        match file.read_line(&mut buffer) {
            Err(e) => bail!(e),
            Ok(0) => break,
            Ok(result) => {
                num_lines += 1;
                num_bytes += result;
                num_chars += buffer.chars().count();
                num_words += buffer.split_whitespace().count();
            }
        }
    }

    Ok(Counts {
        num_lines,
        num_bytes,
        num_chars,
        num_words,
    })
}

fn run(mut args: Args) -> Result<()> {
    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut toto = Counts::default();

    for filename in &args.files {
        match open(filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                let info = count(file)?;
                show_me_the_mony(args.chars, &info, filename);
                toto += info;
            }
        }
    }

    if args.files.len() > 1 {
        show_me_the_mony(args.chars, &toto, "total");
    }

    Ok(())
}

fn show_me_the_mony(should_show_chars: bool, file_info: &Counts, filename: &str) {
    println!(
        "{:8} {:8} {:8} {}",
        file_info.num_lines,
        file_info.num_words,
        if should_show_chars {
            file_info.num_chars
        } else {
            file_info.num_bytes
        },
        if filename != "-" { filename } else { "" }
    );
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{Counts, count};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world.\nI just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = Counts {
            num_lines: 2,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
