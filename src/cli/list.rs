use std::path::PathBuf;

use clap::Args;

use crate::cli::CLI;
use crate::overlays::repository::Repository;


#[derive(Args, Debug)]
pub struct Options {
    // #[structopt(short, long, help = "Count all characters in the string")]
    // length: bool,
    // #[structopt(short, long, help = "Count only numbers in the given string")]
    // numbers: bool,
    // #[structopt(short, long, help = "Count all spaces in the string")]
    // spaces: bool
}

pub fn execute(cli: &CLI, args: &Options) {
    if cli.debug {
        println!("{:#?}", args);
    }
    
    // let repo = Repository::new( PathBuf::from(&cli.home) );

    for overlay in Repository::new( PathBuf::from(&cli.home) ).overlays() {
        println!("{}", overlay.name);
    }
}

