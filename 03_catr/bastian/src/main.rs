use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;
use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("bastian")
        .about("Rust version of cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number-nonblank"),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number-nonblank"),
    }
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Ok(content) => {
                let mut line_num = 0;
                for line in content.lines() {
                    match line {
                        Ok(l) => {
                            if args.number_lines {
                                line_num += 1;
                                println!("{:>6}\t{l}", line_num);
                            } else if args.number_nonblank_lines {
                                if l.is_empty() {
                                    println!();
                                } else {
                                    line_num += 1;
                                    println!("{:>6}\t{l}", line_num);
                                }
                            } else {
                                println!("{l}");
                            }
                        }
                        Err(err) => println!("Cannot read line: {err}"),
                    }
                }
            }
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
