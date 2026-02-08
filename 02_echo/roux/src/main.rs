//use std::env::args;
use clap::{Arg, ArgAction, Command};

fn main() {
    // println!("{:?}", args());
    let matches = Command::new("echor") // create new instance of App
        .version("0.1.0") // use semantic versioning
        .author("Roux Buciu") // include author info
        .about("Rust version of echo") // a short description of the program
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches(); // parse the arguments

    // must specify the type of vec, because iterator can be of a bunch of types
    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();
    // rust infers where it can
    let omit_newline = matches.get_flag("omit_newline");

    // works
    // let mut ending = "\n";
    // if omit_newline {
    //     ending = "";
    // }

    // more Rust like
    // let ending = if omit_newline { "" } else { "\n" };

    // print!("{}{}", text.join(" "), ending);

    // more functional
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    // println!("{:#?}", matches)
}
