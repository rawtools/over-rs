use std::path::PathBuf;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use console::Emoji;
use git2::Repository;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::{OwoColorize, colors::*};

use crate::{exec::{Action, Ctx}, ui::{self, style}};


static CLONE: Emoji<'_, '_> = Emoji("", "");

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

#[async_trait]
impl Action for EnsureGitRepository {
    async fn execute(&self, ctx: Ctx) -> Result<()> {
        if ctx.verbose || ctx.dry_run {
            ui::info(  
                format!("{} {} {} {} {}",
                    CLONE,
                    "clone:".fg::<White>(), 
                    self.remote,
                    "->".fg::<White>(), 
                    self.path.display(),
                )
            )?;
        }
        let state = ctx.state.read().unwrap();
        let pb = match state.progress.as_ref() {
            Some(progress) => Some(
                                progress.add(
                                    ProgressBar::new(100).with_style(
                                        ProgressStyle::default_bar()
                                            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {wide_msg:.red}{bytes}/{total_bytes} ({eta})")
                                            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                                    ).with_message("Clone")
                                )
                            ),
                    // .with_key("eta", |state| format!("{:.1}s", state.eta().as_secs_f64())
                // ProgressStyle::default_spinner()
            // .template("{prefix:.bold.dim} {spinner} {wide_msg}")
            _ => None,
        };

        if !ctx.dry_run {
            if self.path.exists() {
                if let Some(ref p) = pb {
                    p.abandon_with_message("Repository exists");
                } else if ctx.verbose {
                    println!("repo exists");
                }
                Repository::open(self.path.as_path())?
            } else {
                match Repository::clone_recurse(&self.remote, self.path.as_path()) {
                    Ok(repo) => repo,
                    Err(e) => {
                        if let Some(p) = pb {
                            // p.println(e.to_string());
                            p.abandon_with_message(e.to_string());
                        }
                        return Err(anyhow!(e))
                    },
                }
            };
            // for entry in &repo.config()?.entries(None).unwrap() {
            //     let entry = entry.unwrap();
            //     println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
            // }
        }
        if let Some(p) = pb {
            p.finish_with_message("Done");
        }

        Ok(())
    }

    // fn display(&self, _ctx: &Context) -> Result<String> {
    //     Ok(format!(" {} {} -> {}", 
    //         "git repository:".fg::<White>(), 
    //         self.remote,
    //         self.path.display(),
    //     ))
    // }
}
