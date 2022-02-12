use clap::Args;

use crate::cli::CLI;

#[derive(Args, Debug)]
pub struct Options {
    // #[structopt(short, long, help = "Transforms a string to uppercase")]
    // upper: bool,
    // #[structopt(short, long, help = "Transforms a string to lowercase")]
    // lower: bool,
    // #[structopt(short, long, help = "Reverses a string")]
    // reverse: bool,
    // #[structopt(short="pref", long, help = "Adds a prefix to the string")]
    // prefix: Option<String>,
    // #[structopt(short="suf", long, help = "Adds a suffix to the string")]
    // suffix: Option<String>,
}


pub fn execute(cli: &CLI, args: &Options) {
    println!("Apply called");
    if cli.debug {
        println!("{:#?}", args);
    }
}
