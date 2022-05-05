use std::path::PathBuf;

use crate::Expect;
use crate::cli::CLI;
use crate::overlays::repository::Repository;

pub async fn execute(cli: &CLI) -> Expect<()> {
    let repo = Repository::new( PathBuf::from(&cli.home) );
    println!("{:#?}", repo);
    Ok(())
}

