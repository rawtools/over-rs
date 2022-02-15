use std::collections::HashMap;
use std::path::{Path, PathBuf};
  
use config::{Config, File};
use globset::GlobBuilder;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use tera::{Context, Tera};

use crate::Expect;

use super::{Repository, pattern};


#[derive(Debug, Deserialize, Serialize)]
pub struct Overlay {
    pub name: String,
    
    pub root: PathBuf,

    // #[serde(skip)]
    // pub parent: Option<Box<Overlay>>,
    
    // // #[serde(default = "default_is_not_parent")]
    // // pub parent: bool,

    pub description: Option<String>,

    pub target: String,

    pub exclude: Option<Vec<String>>,

    pub git: Option<HashMap<String, String>>,

    pub install: Option<HashMap<String, Vec<String>>>,
}


impl Overlay {
    pub fn new(repository: &Repository, root: &Path) -> Expect<Self> {
        let mut s = Config::new();
        let mut dirs: Vec<PathBuf> = Vec::new();
        let mut dir = root;
        loop {
            dirs.push(dir.to_path_buf());
            if dir == repository.root {
                break
            }
            dir = dir.parent().unwrap();
        }
        
        for dir in dirs.iter().rev() {
            let basename = dir.as_path().join("over");
            s.merge(File::with_name(basename.to_str().unwrap()).required(dir == root))?;    
        }

        let name = root.strip_prefix(repository.root.as_path())?.to_str().unwrap();
        // s.merge(File::with_name(root.join("over").to_str().unwrap()))?;
        s.set("name", name)?;
        s.set("root", root.to_str())?;
        s.set_default("target", "~")?;
        Ok(s.try_into()?)
    }

    pub fn apply(&self) -> Expect<()> {

        let mut ctx = Context::new();
        ctx.insert("overlay", &self);
        let target = Tera::one_off(self.target.as_str(), &ctx, true)?;

        println!("Apply to: {}", target);
        let exclude = GlobBuilder::new(&pattern()).literal_separator(true).build()?.compile_matcher();
        let files = WalkDir::new(&self.root)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !exclude.is_match(e.path()));
            
        for file in files {
            println!("{:#?}", file);
        }
        
        Ok(())
    }
}
