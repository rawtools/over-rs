use std::error::Error;
use std::path::PathBuf;

use clap::Args;
use owo_colors::{OwoColorize, colors::*};

use crate::cli::CLI;
use crate::overlays::Repository;

#[derive(Args, Debug)]
pub struct Params {
    #[clap(help = "Name of the overlay to display")]
    name: String,
}


pub fn execute(cli: &CLI, args: &Params) -> Result<(), Box<dyn Error>> {
    if cli.debug {
        println!("{:#?}", cli);
        println!("{:#?}", args);
    }

    let repo = Repository::new(PathBuf::from(&cli.home));
    let overlay = repo.get(&args.name)?;
    
    println!("🌟 {} 🌟", overlay.name.fg::<White>().bold());
    println!("overlay: {:#?}", overlay);
    Ok(())
}
