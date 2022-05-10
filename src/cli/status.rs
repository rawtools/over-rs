use std::path::PathBuf;

use anyhow::Result;
use crate::cli::CLI;
use crate::overlays::repository::Repository;

pub async fn execute(cli: &CLI) -> Result<()> {
    let repo = Repository::new( PathBuf::from(&cli.home) );
    println!("{:#?}", repo);
    Ok(())
}

