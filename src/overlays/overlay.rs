use std::collections::HashMap;
use std::path::{Path, PathBuf};

use dirs::home_dir;
use config::{Config, File, FileFormat, FileSourceFile};
use globset::GlobBuilder;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use tera::{Context, Tera};

use crate::actions::{EnsureLink, EnsureDir, EnsureGitRepository};
use crate::{Expect, exec};
use crate::exec::Action;

use super::{Repository, pattern};


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
    pub fn new(repository: &Repository, root: &Path) -> Expect<Self> {
        let name = root.strip_prefix(repository.root.as_path())?.to_str().unwrap();
        let mut sources: Vec<File<FileSourceFile, FileFormat>> = Vec::new();
        let mut dir = root;
        loop {
            let basename = dir.join("over");
            sources.push(File::with_name(basename.to_str().unwrap()).required(dir == root));
            if dir == repository.root {
                break
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

    pub fn resolve_target(&self, ctx: &exec::Context) -> Expect<PathBuf> {
        let path = PathBuf::from(
            &Tera::one_off(
                self.target.as_str(), 
                &Context::from_serialize(ctx)?,
                true,
            )?
        );
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
            _ => home_dir().map(|mut h| {
                if h == Path::new("/") {
                    // Corner case: `h` root directory;
                    // don't prepend extra `/`, just drop the tilde.
                    path.strip_prefix("~").unwrap().to_path_buf()
                } else {
                    h.push(path.strip_prefix("~/").unwrap());
                    h
                }
            }).unwrap(),
        })
            
    }


    pub fn apply_to(&self, ctx: &exec::Context, target_root: &Path) -> Expect<()> {
        // let target_root = self.resolve_target(&ctx)?;

        if !target_root.exists() {
            let mkdir = EnsureDir::new(target_root.to_path_buf());
            mkdir.execute(ctx)?;
        }

        if let Some(git_repos) = &self.git {
            for git_repo in git_repos {
                let action = EnsureGitRepository::new(target_root.join(git_repo.0), git_repo.1.to_string());
                action.execute(ctx)?;
            }
        }

        let exclude = GlobBuilder::new(&pattern()).literal_separator(true).build()?.compile_matcher();
        let files = WalkDir::new(&self.root)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !exclude.is_match(e.path()));

        for file in files {
            let rel_path = file.path().strip_prefix(&self.root)?;
            let target = target_root.join(rel_path);
            let path = file.path();
            let action: Box<dyn Action> = match () {
                _ if path.is_dir() => Box::new(EnsureDir::new(target)),
                _ if path.is_file() => Box::new(EnsureLink::new(file.clone().into_path(), target)),
                _ => Box::new(EnsureLink::new(file.clone().into_path(), target)),
            };
            action.execute(ctx)?;
        }
        
        Ok(())
    }

    pub fn apply(&self, ctx: &exec::Context) -> Expect<()> {
        let target_root = self.resolve_target(&ctx)?;
        self.apply_to(ctx, &target_root)
    }
}
