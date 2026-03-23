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
        .author("adudenamedruby")
        .about("Rust version of cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s) [default: -]")
                // .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .help("Number lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .help("Number non-blank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let files = matches
        .get_many("files")
        .map(|vals| vals.cloned().collect())
        .unwrap_or_else(|| vec!["-".to_string()]);

    let number_lines = matches.get_flag("number_lines");
    let number_nonblank_lines = matches.get_flag("number_nonblank_lines");

    if (number_lines == number_nonblank_lines) && number_lines {
        eprintln!("error: the argument '--number-nonblank' cannot be used with '--number'");
        std::process::exit(1)
    }

    Args {
        files,
        number_lines,
        number_nonblank_lines,
    }
}

fn main() {
    let args = get_args();
    println!("{args:#?}");
}
