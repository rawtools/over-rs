use std::path::PathBuf;

use clap::Args;

use anyhow::Result;
use crate::cli::CLI;
use crate::overlays::Repository;
use crate::ui::style;

#[derive(Args, Debug)]
pub struct Params {
    #[clap(help = "Name of the overlay to display")]
    name: String,
}


pub async fn execute(cli: &CLI, args: &Params) -> Result<()> {
    if cli.debug {
        println!("{:#?}", cli);
        println!("{:#?}", args);
    }

    let repo = Repository::new(PathBuf::from(&cli.home));
    let overlay = repo.get(&args.name)?;
    
    println!("🌟 {} 🌟", style::white_b(&overlay.name));
    println!("overlay: {:#?}", overlay);
    Ok(())
}
