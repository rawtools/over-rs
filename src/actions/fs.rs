use std::fmt;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use globset::GlobBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use symlink::symlink_file;

use walkdir::WalkDir;

use crate::exec::{Action, Ctx};
use crate::overlays::{self, Overlay};
use crate::ui::{self, emojis, style};

static SPINNER_STYLE: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::with_template("{spinner:.cyan} {wide_msg}")
        .unwrap()
        .tick_chars(style::TICK_CHARS_BRAILLE_4_6_DOWN.as_str())
});

pub async fn link(ctx: Ctx, overlay: &Overlay, to: &Path) -> Result<()> {
    ui::info(format!(
        "{} {}",
        emojis::LINK,
        style::white("Linking files"),
    ))?;

    let progress = ProgressBar::new_spinner()
        .with_style(SPINNER_STYLE.clone())
        .with_message("");

    let exclude = GlobBuilder::new(&overlays::GLOB_PATTERN)
        .literal_separator(true)
        .build()?
        .compile_matcher();
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
            _ if path.is_file() => Box::new(EnsureLink::new(
                overlay.clone(),
                file.clone().into_path(),
                target,
            )),
            _ => Box::new(EnsureLink::new(
                overlay.clone(),
                file.clone().into_path(),
                target,
            )),
        };
        if ctx.verbose || ctx.dry_run {
            progress.println(format!("{}", action));
        }
        progress.set_message(format!("{}", action));
        action.execute(ctx.clone()).await?;
    }
    // progress.finish_with_message("DOne");
    progress.finish_and_clear();
    Ok(())
}

pub struct EnsureLink {
    pub overlay: Overlay,
    pub source: PathBuf,
    pub target: PathBuf,
}

impl EnsureLink {
    pub fn new(overlay: Overlay, source: PathBuf, target: PathBuf) -> Self {
        Self {
            overlay,
            source,
            target,
        }
    }
}

impl fmt::Display for EnsureLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{} -> {}", self.source.display(), self.target.display())
        // We operate on string as path normalization is broken in rust
        // See:
        //  - https://users.rust-lang.org/t/trailing-in-paths/43166/9
        //  - https://github.com/rust-lang/rfcs/issues/2208
        let rel_path = self
            .source
            .to_str()
            .unwrap()
            .strip_prefix(self.overlay.root.to_str().unwrap())
            .unwrap();
        let target_root = self
            .target
            .to_str()
            .unwrap()
            .strip_suffix(rel_path)
            .unwrap();
        write!(
            f,
            "{} {} {}{} {} {}{}{}",
            emojis::LINK,
            style::white("link:"),
            style::white("{"),
            self.overlay.root.display(),
            style::white("->"),
            target_root,
            style::white("}"),
            rel_path,
        )
    }
}

#[async_trait]
impl Action for EnsureLink {
    async fn execute(&self, ctx: Ctx) -> Result<()> {
        if self.target.exists() {
            let src = fs::read_link(self.target.as_path())?;
            if src != self.source {
                // TODO: handle links exists
            }
        } else if !ctx.dry_run {
            symlink_file(self.source.as_path(), self.target.as_path())?;
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
        write!(
            f,
            "{} {} {}",
            emojis::DIRECTORY,
            style::white("create directory:"),
            self.path.display(),
        )
    }
}

#[async_trait]
impl Action for EnsureDir {
    async fn execute(&self, ctx: Ctx) -> Result<()> {
        if !ctx.dry_run {
            create_dir_all(self.path.as_path())?;
        }
        Ok(())
    }
}
