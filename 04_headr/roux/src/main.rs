use clap::{Arg, Command};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

fn get_args() -> Args {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("adudenamedruby")
        .about("Rust version of `head`")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .help("Number of lines")
                .short('n')
                .long("lines")
                .value_parser(clap::value_parser!(u64).range(1..))
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .short('c')
                .long("bytes")
                .value_parser(clap::value_parser!(u64).range(1..)),
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        lines: matches.get_one("lines").cloned().unwrap(),
        bytes: matches.get_one("bytes").cloned(),
    }
}

fn run(args: Args) {
    print!("{args:#?}");
}

fn main() {
    let args = get_args();
    run(args);
}
