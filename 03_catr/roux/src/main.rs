use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    numben_nonblank_lines: bool,
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
                .required(false)
                .num_args(1..),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Number lines"),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .action(ArgAction::SetTrue)
                .help("Number non-blank lines"),
        )
        .get_matches();

    Args {
        files: vec!["-".to_string()],
        number_lines: false,
        numben_nonblank_lines: false,
    }
}

fn main() {
    let args = get_args();
    println!("{args:#?}");
}
