use std::error::Error;
use std::path::{PathBuf, Path};

use clap::Args;
use dirs::home_dir;
use owo_colors::{OwoColorize, colors::*};

use crate::cli::CLI;
use crate::exec::Context;
use crate::overlays::Repository;

#[derive(Args, Debug)]
pub struct Params {
    #[clap(help = "Name of the overlay to apply")]
    name: String,

    #[clap(short, long, help = "Name of the overlay to apply")]
    target: Option<String>,
    
}


pub fn execute(cli: &CLI, args: &Params) -> Result<(), Box<dyn Error>> {
    if cli.debug {
        println!("{:#?}", cli);
        println!("{:#?}", args);
    }

    let repo = Repository::new(PathBuf::from(&cli.home));
    let overlay = repo.get(&args.name)?;
    
    let ctx = Context {
        dry_run: cli.dry_run,
        debug: cli.debug,
        verbose: cli.verbose,
        repository: Some(repo),
        overlay: Some(overlay.clone()),
    };

    let target = match &args.target {
        Some(root) => PathBuf::from(&root),
        None => overlay.resolve_target(&ctx)?,
    };
    
    println!("📦 {} {} {} {}", 
        "Applying overlay".fg::<White>().bold(), 
        overlay.name.fg::<White>().bold().italic(),
        "to".fg::<White>().bold(), 
        target.to_str().unwrap().fg::<White>().bold().italic(),
    );

    match &args.target {
        Some(root) => overlay.apply_to(&ctx, Path::new(&root))?,
        None => overlay.apply(&ctx)?,
    }
    
    Ok(())
}
