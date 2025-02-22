use std::path::PathBuf;

use crate::cli::CLI;
use crate::overlays::repository::Repository;
use anyhow::Result;

pub async fn execute(cli: &CLI) -> Result<()> {
    let repo = Repository::new(PathBuf::from(&cli.home));
    println!("{:#?}", repo);
    Ok(())
}
