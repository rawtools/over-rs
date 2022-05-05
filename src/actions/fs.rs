use std::{path::PathBuf, fmt, fs::{create_dir_all, self}};

use async_trait::async_trait;
use symlink::symlink_file;

use owo_colors::{OwoColorize, colors::*};

use crate::{exec::{Action, Context}, Expect};

pub struct EnsureLink {
    pub source: PathBuf,
    pub target: PathBuf,
}

impl EnsureLink {
    pub fn new(source: PathBuf, target: PathBuf) -> Self {
        Self {
            source,
            target,
        }
    }
} 

impl fmt::Display for EnsureLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source.display(), self.target.display())
    }
}

#[async_trait]
impl Action for EnsureLink {
    async fn execute(&self, ctx: &Context) -> Expect<()> {
        let overlay = ctx.overlay.as_ref().unwrap();

        

        if ctx.verbose || ctx.dry_run {
            // We operate on string as path normalization is broken in rust
            // See:  
            //  - https://users.rust-lang.org/t/trailing-in-paths/43166/9
            //  - https://github.com/rust-lang/rfcs/issues/2208
            let rel_path = self.source.to_str().unwrap().strip_prefix(&overlay.root.to_str().unwrap()).unwrap();
            let target_root = self.target.to_str().unwrap().strip_suffix(rel_path).unwrap();
            println!("🔗 {} {{{} -> {}}}{}", 
                "link:".fg::<White>(), 
                overlay.root.display(),
                target_root,
                rel_path,
            )
        }

        if self.target.exists() {
            let src = fs::read_link(self.target.as_path())?;
            if src != self.source {
                // TODO: handle links exsists
            }
        } else {
            if !ctx.dry_run {
                symlink_file(self.source.as_path(), self.target.as_path())?;
            }
        }


        Ok(())
        // todo!();
    }

    // fn display(&self, ctx: &Context) -> Expect<String> {
    //     let overlay = ctx.overlay.as_ref().unwrap();
    //     // We operate on string as path normalization is broken in rust
    //     // See:  
    //     //  - https://users.rust-lang.org/t/trailing-in-paths/43166/9
    //     //  - https://github.com/rust-lang/rfcs/issues/2208
    //     let rel_path = self.source.to_str().unwrap().strip_prefix(&overlay.root.to_str().unwrap()).unwrap();
    //     let target_root = self.target.to_str().unwrap().strip_suffix(rel_path).unwrap();
    //     Ok(format!("🔗 {} {{{} -> {}}}{}\n", 
    //         "link:".fg::<White>(), 
    //         overlay.root.display(),
    //         target_root,
    //         rel_path,
    //     ))
    // }
}

pub struct EnsureDir {
    pub path: PathBuf,
    // pub target: PathBuf,
}

impl EnsureDir {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
} 

impl fmt::Display for EnsureDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mkdri {}", self.path.display())
    }
}

#[async_trait]
impl Action for EnsureDir {
    async fn execute(&self, ctx: &Context) -> Expect<()> {
        


        if ctx.verbose || ctx.dry_run {
            println!("📁 {} {}", 
                "create directory:".fg::<White>(), 
                self.path.display(),
            )
        }

        if !ctx.dry_run {
            create_dir_all(self.path.as_path())?;
        }
        Ok(())
    }

    // fn display(&self, _ctx: &Context) -> Expect<String> {
    //     // let overlay = ctx.overlay.as_ref().unwrap();
    //     // let rel_path = self.source.strip_prefix(overlay.root.as_path())?;
    //     // let target_root = self.target.to_str().unwrap().strip_suffix(rel_path.to_str().unwrap()).unwrap();
    //     Ok(format!("📁 {} {}\n", 
    //         "create directory:".fg::<White>(), 
    //         self.path.display(),
    //     ))
    // }
}
