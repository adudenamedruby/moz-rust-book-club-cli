use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

/* derive version of this code
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
        )]
    number_lines: bool,

    #[arg(
        short('b'),
        long("number-nonblank")
        )]
    number_nonblank_lines: bool,
}
*/

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("adudenamedruby")
        .about("Rust version of cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                // this is the proper way to provide a default value!!
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Roux - This is how I originally provided a default for files
    // let files = matches
    //     .get_many("files")
    //     .map(|vals| vals.cloned().collect())
    //     .unwrap_or_else(|| vec!["-".to_string()]);

    // Roux - this is how I originally made sure these two arguments don't conflict
    // if (number_lines == number_nonblank_lines) && number_lines {
    //     eprintln!("error: the argument '--number-nonblank' cannot be used with '--number'");
    //     std::process::exit(1)
    // }

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(_) => println!("Opened {filename}"),
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

fn main() {
    // Derive version
    // let args = Argss::parse();

    // let args = get_args();
    // println!("{args:#?}");

    if let Err(e) = run(get_args()) {
        // alternatively run(Args::parse())
        eprintln!("{e}");
        std::process::exit(1);
    }
}
