use std::path::PathBuf;

use clap::Args;

use crate::cli::CLI;
use crate::exec::Context;
use crate::overlays::Repository;
use crate::ui::{emojis, style};
use anyhow::Result;

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
        println!("{}", e);
    }

    // match &args.target {
    //     Some(root) => overlay.apply_to(&ctx, Path::new(&root)).await?,
    //     None => overlay.apply(&ctx).await?,
    // }

    Ok(())
}
