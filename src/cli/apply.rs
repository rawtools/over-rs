use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::cli::CLI;
use crate::exec::Context;
use crate::overlays::Repository;
use crate::ui::{emojis, style};

#[derive(Args, Debug)]
pub struct Params {
    #[clap(help = "Name of the overlay to apply")]
    name: String,

    #[clap(short, long, help = "The target directory")]
    target: Option<String>,

    #[clap(long, short = 'n', help = "Run without applying changes")]
    dry_run: bool,

    #[clap(long, short, help = "Overwrite without prompting")]
    force: bool,
}

pub async fn execute(cli: &CLI, args: &Params) -> Result<()> {
    if cli.debug {
        println!("{:#?}", cli);
    }

    let repo = Repository::new(PathBuf::from(&cli.home));
    if cli.debug {
        println!("{:#?}", repo);
    }
    let overlay = repo.get(&args.name)?;
    if cli.debug {
        println!("{:#?}", overlay);
    }

    let ctx = Context::new(
        args.dry_run,
        cli.debug,
        cli.verbose,
        args.force,
        repo,
        Some(overlay.clone()),
    );

    let target = match &args.target {
        Some(root) => PathBuf::from(&root),
        None => overlay.resolve_target(&ctx)?,
    };

    let result = overlay.apply_to(&ctx, &target).await;
    if let Err(e) = result {
        println!(
            "{} {} {} {} {}",
            emojis::CROSSMARK,
            style::white_b("Failed to apply overlay"),
            style::white_bi(&overlay.name),
            style::white_b("to"),
            style::white_bi(target.to_str().unwrap()),
        );
        println!("{:#?}", e);
    }

    // match &args.target {
    //     Some(root) => overlay.apply_to(&ctx, Path::new(&root)).await?,
    //     None => overlay.apply(&ctx).await?,
    // }

    Ok(())
}
