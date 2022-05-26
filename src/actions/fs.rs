use std::path::{PathBuf, Path};
use std::fmt;
use std::fs::{create_dir_all, self};

use anyhow::Result;
use async_trait::async_trait;
use globset::GlobBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use symlink::symlink_file;

use owo_colors::{OwoColorize, colors::*};
use walkdir::WalkDir;

use crate::exec::{Action, Ctx};
use crate::overlays;
use crate::ui::{self, emojis, style};

lazy_static! {

    static ref SPINNER_STYLE: ProgressStyle = ProgressStyle
        ::with_template("{spinner:.cyan} {wide_msg}").unwrap()
        .tick_chars(style::TICK_CHARS_BRAILLE_4_6_DOWN.as_str());
        // .progress_chars(style::THIN_PROGRESS.as_str());

}

pub async fn link(ctx: Ctx, to: &Path) -> Result<()> {
    if let Some(overlay) = &ctx.overlay {
        ui::info(format!("{} {}",
            emojis::LINK,
            style::WHITE.apply_to("Linking files"), 
        ))?;
        
        let progress = ProgressBar::new_spinner().with_style(SPINNER_STYLE.clone()).with_message("");
        
        let exclude = GlobBuilder::new(&overlays::GLOB_PATTERN).literal_separator(true).build()?.compile_matcher();
        let files = WalkDir::new(&overlay.root)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !exclude.is_match(e.path()));
    
        for file in files {
            // progress.tick();
            let rel_path = file.path().strip_prefix(&overlay.root)?;
            let target = to.join(rel_path);
            let path = file.path();
            let action: Box<dyn Action> = match () {
                _ if path.is_dir() => Box::new(EnsureDir::new(target)),
                _ if path.is_file() => Box::new(EnsureLink::new(file.clone().into_path(), target)),
                _ => Box::new(EnsureLink::new(file.clone().into_path(), target)),
            };
            progress.set_message(format!("{}", action));
            action.execute(ctx.clone()).await?;
        }
        // progress.finish_with_message("DOne");
        progress.finish_and_clear();
    };
    Ok(())
}


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
    async fn execute(&self, ctx: Ctx) -> Result<()> {
        let overlay = ctx.overlay.as_ref().unwrap();

        if ctx.verbose || ctx.dry_run {
            // We operate on string as path normalization is broken in rust
            // See:  
            //  - https://users.rust-lang.org/t/trailing-in-paths/43166/9
            //  - https://github.com/rust-lang/rfcs/issues/2208
            let rel_path = self.source.to_str().unwrap().strip_prefix(&overlay.root.to_str().unwrap()).unwrap();
            let target_root = self.target.to_str().unwrap().strip_suffix(rel_path).unwrap();
            println!("{} {} {}{} {} {}{}{}",
                emojis::LINK, 
                "link:".fg::<White>(),
                "{".fg::<White>(), 
                overlay.root.display(),
                "->".fg::<White>(),
                target_root,
                "}".fg::<White>(),
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
    }
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
    async fn execute(&self, ctx: Ctx) -> Result<()> {
        
        if ctx.verbose || ctx.dry_run {
            println!("{} {} {}", 
                emojis::DIRECTORY,
                "create directory:".fg::<White>(), 
                self.path.display(),
            )
        }

        if !ctx.dry_run {
            create_dir_all(self.path.as_path())?;
        }
        Ok(())
    }
}
