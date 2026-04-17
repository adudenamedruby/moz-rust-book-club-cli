use anyhow::Result;
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
    #[arg(short = 'm', long, conflicts_with = "bytes", default_value_t = true)]
    chars: bool,
}

fn run(mut args: Args) -> Result<()> {
    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    println!("{args:#?}");
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
