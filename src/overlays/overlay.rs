use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use config::{Config, File, FileFormat, FileSourceFile};
use dirs::home_dir;
use serde::{Deserialize, Serialize};

use tera::{Context, Tera};

use crate::actions::{self, EnsureDir};
use crate::exec::{self, Action, Ctx};
use crate::ui::{emojis, style};

use super::Repository;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Overlay {
    pub name: String,

    pub root: PathBuf,

    // #[serde(skip)]
    // pub parent: Option<Box<Overlay>>,

    // // #[serde(default = "default_is_not_parent")]
    // // pub parent: bool,
    pub description: Option<String>,

    pub target: String,

    pub uses: Option<Vec<String>>,

    pub exclude: Option<Vec<String>>,

    pub git: Option<HashMap<String, String>>,

    pub install: Option<HashMap<String, Vec<String>>>,
}

impl Overlay {
    pub fn new(repository: &Repository, root: &Path) -> Result<Self> {
        let name = root
            .strip_prefix(repository.root.as_path())?
            .to_str()
            .unwrap();
        let mut sources: Vec<File<FileSourceFile, FileFormat>> = Vec::new();
        let mut dir = root;
        loop {
            let basename = dir.join("over");
            sources.push(File::with_name(basename.to_str().unwrap()).required(dir == root));
            if dir == repository.root {
                break;
            }
            dir = dir.parent().unwrap();
        }

        let s = Config::builder()
            .add_source(sources)
            .set_override("name", name)?
            .set_override("root", root.to_str())?
            .set_default("target", "~")?
            .build()?;

        Ok(s.try_deserialize()?)
    }

    pub fn resolve_target(&self, ctx: &exec::Context) -> Result<PathBuf> {
        let path = PathBuf::from(&Tera::one_off(
            self.target.as_str(),
            &Context::from_serialize(ctx)?,
            true,
        )?);
        // if !path.starts_with("~") {
        //     return Ok(path);
        // }
        // if path == Path::new("~") {
        //     return Ok(home_dir().unwrap());
        // }
        // Ok(home_dir().map(|mut h| {
        //     if h == Path::new("/") {
        //         // Corner case: `h` root directory;
        //         // don't prepend extra `/`, just drop the tilde.
        //         path.strip_prefix("~").unwrap().to_path_buf()
        //     } else {
        //         h.push(path.strip_prefix("~/").unwrap());
        //         h
        //     }
        // }).unwrap())

        Ok(match path.to_str().unwrap() {
            p if !p.starts_with("~") => path,
            "~" => home_dir().unwrap(),
            _ => home_dir()
                .map(|mut h| {
                    if h == Path::new("/") {
                        // Corner case: `h` root directory;
                        // don't prepend extra `/`, just drop the tilde.
                        path.strip_prefix("~").unwrap().to_path_buf()
                    } else {
                        h.push(path.strip_prefix("~/").unwrap());
                        h
                    }
                })
                .unwrap(),
        })
    }

    pub async fn apply_to(&self, ctx: &Ctx, target_root: &Path) -> Result<()> {
        if !target_root.exists() {
            let mkdir = EnsureDir::new(target_root.to_path_buf());
            mkdir.execute(ctx.clone()).await?;
        }
        println!(
            "{} {} {} {} {}",
            emojis::PACKAGE,
            style::white_b("Applying overlay"),
            style::white_bi(&self.name),
            style::white_b("to"),
            style::white_bi(target_root.to_str().unwrap()),
        );
        if let Some(uses) = &self.uses {
            for name in uses {
                let overlay = ctx.repository.get(name).expect("failed");
                let _ = Box::pin(overlay.apply_to(ctx, target_root)).await;
            }
        }

        actions::git::clone_repositories(ctx.clone(), self, target_root).await?;
        actions::fs::link(ctx.clone(), self, target_root).await?;

        println!(
            "{} {} {} {} {} {}",
            emojis::SPARKLE,
            style::white_b("Applied overlay"),
            style::white_bi(&self.name),
            style::white_b("to"),
            style::white_bi(target_root.to_str().unwrap()),
            style::white_b("with success"),
        );

        Ok(())
    }

    pub async fn apply(&self, ctx: &Ctx) -> Result<()> {
        let target_root = self.resolve_target(ctx)?;
        self.apply_to(ctx, &target_root).await
    }
}
