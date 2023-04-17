use std::path::PathBuf;

use clap::Args;

use anyhow::Result;
use crate::cli::CLI;
use crate::exec::{Context};
use crate::overlays::Repository;
use crate::ui::{emojis, style};

#[derive(Args, Debug)]
pub struct Params {
    #[clap(help = "Name of the overlay to apply")]
    name: String,

    #[clap(short, long, help = "The target directory")]
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
        style::white_b("Applying overlay"), 
        style::white_bi(&overlay.name),
        style::white_b("to"), 
        style::white_bi(target.to_str().unwrap()),
    );

    let result = overlay.apply_to(&ctx, &target).await;
    if let Err(e) = result {
        println!("{} {} {} {} {}", 
            emojis::CROSSMARK,
            style::white_b("Failed to apply overlay"), 
            style::white_bi(overlay.name),
            style::white_b("to"), 
            style::white_bi(target.to_str().unwrap()),
        );
        println!("{}", e);
    } else {
        println!("{} {} {} {} {} {}", 
            emojis::SPARKLE,
            style::white_b("Applied overlay"), 
            style::white_bi(&overlay.name),
            style::white_b("to"), 
            style::white_bi(target.to_str().unwrap()),
            style::white_b("with success"), 
        );
    };

    // match &args.target {
    //     Some(root) => overlay.apply_to(&ctx, Path::new(&root)).await?,
    //     None => overlay.apply(&ctx).await?,
    // }
    
    Ok(())
}
