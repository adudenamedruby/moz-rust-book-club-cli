use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author = "adudenamedruby", version, about)]
struct Args {
    /// Input file
    #[arg(default_value = "-")]
    in_file: String,

    /// Output file
    #[arg()]
    out_file: Option<String>,

    /// Shows count
    #[arg(short, long, default_value_t = false)]
    count: bool,
}

fn run(args: Args) -> Result<()> {
    println!("{args:?}");
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
