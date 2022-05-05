use std::{path::PathBuf};

use owo_colors::{OwoColorize, colors::*};

use crate::{exec::{Action, Context}, Expect};

pub struct EnsureGitRepository {
    pub path: PathBuf,
    pub remote: String,
}

impl EnsureGitRepository {
    pub fn new(path: PathBuf, remote: String) -> Self {
        Self { path, remote }
    }
} 

// impl fmt::Display for EnsureGitRepository {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} -> {}", self.source.display(), self.target.display())
//     }
// }

impl Action for EnsureGitRepository {
    fn execute(&self, ctx: &Context) -> Expect<()> {
        if ctx.verbose || ctx.dry_run {
            println!(" {} {} {} {}", 
                "clone:".fg::<White>(), 
                self.remote,
                "->".fg::<White>(), 
                self.path.display(),
            ) 
        }
        if !ctx.dry_run {
            // todo!();
        }

        Ok(())
    }

    // fn display(&self, _ctx: &Context) -> Expect<String> {
    //     Ok(format!(" {} {} -> {}", 
    //         "git repository:".fg::<White>(), 
    //         self.remote,
    //         self.path.display(),
    //     ))
    // }
}
