use std::path::PathBuf;

use clap::Args;
use owo_colors::{OwoColorize, colors::*};

use anyhow::Result;
use crate::cli::CLI;
use crate::exec::{Context};
use crate::overlays::Repository;
use crate::ui::emojis;

#[derive(Args, Debug)]
pub struct Params {
    #[clap(help = "Name of the overlay to apply")]
    name: String,

    #[clap(short, long, help = "Name of the overlay to apply")]
    target: Option<String>,
    
}


pub async fn execute(cli: &CLI, args: &Params) -> Result<()> {
    if cli.debug {
        println!("{:#?}", cli);
        println!("{:#?}", args);
    }

    let repo = Repository::new(PathBuf::from(&cli.home));
    let overlay = repo.get(&args.name)?;
    
    let ctx = Context::new(
        cli.dry_run, 
        cli.debug, 
        cli.verbose, 
        Some(repo), 
        Some(overlay.clone()),
    );

    let target = match &args.target {
        Some(root) => PathBuf::from(&root),
        None => overlay.resolve_target(&ctx)?,
    };
    
    println!("{} {} {} {} {}", 
        emojis::PACKAGE,
        "Applying overlay".fg::<White>().bold(), 
        overlay.name.fg::<White>().bold().italic(),
        "to".fg::<White>().bold(), 
        target.to_str().unwrap().fg::<White>().bold().italic(),
    );

    overlay.apply_to(&ctx, &target).await

    // match &args.target {
    //     Some(root) => overlay.apply_to(&ctx, Path::new(&root)).await?,
    //     None => overlay.apply(&ctx).await?,
    // }
    
    // Ok(())
}
