use std::{path::PathBuf, fmt};

use crate::exec::{Action, Context};

pub struct EnsureLink {
    source: PathBuf,
    target: PathBuf,
}

// pub impl EnsureLink {
//     // fn new()
// } 

impl fmt::Display for EnsureLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source.display(), self.target.display())
    }
}

impl Action for EnsureLink {
    fn execute(ctx: Context) {
        todo!()
    }
}
