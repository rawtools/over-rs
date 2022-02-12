use std::path::PathBuf;

use crate::cli::CLI;
use crate::overlays::repository::Repository;

pub fn execute(cli: &CLI) {
    let repo = Repository::new( PathBuf::from(&cli.home) );
    println!("{:#?}", repo);
}

