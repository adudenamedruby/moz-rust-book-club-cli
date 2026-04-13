use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about, long_about=None)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILES", default_value = "-")]
    file: Vec<String>,

    /// Number of lines
    #[arg(
        short = 'n',
        long,
        default_value = "10",
        conflicts_with = "bytes",
        value_parser
    )]
    lines: u64,

    /// Number of bytes
    #[arg(short = 'c', long, value_parser)]
    bytes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args)
}
