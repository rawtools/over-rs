use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
  
use config::{Config, ConfigError, File};
use serde::Deserialize;
use globwalk::GlobWalkerBuilder;


#[derive(Debug, Deserialize)]
pub struct Overlay {
    pub name: String,
    
    pub root: PathBuf,

    // pub parent: Option<Box<Overlay>>,
    
    #[serde(default = "default_is_not_parent")]
    pub parent: bool,

    pub description: Option<String>,

    pub target: Option<String>,

    pub exclude: Option<Vec<String>>,

    pub git: Option<HashMap<String, String>>,

    pub install: Option<HashMap<String, Vec<String>>>,
}

fn default_is_not_parent() -> bool {false}


impl Overlay {
    pub fn new(name: &str, root: &Path) -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(root.join("over").to_str().unwrap()))?;
        s.set("name", name.to_string())?;
        s.set("root", root.to_str())?;
        s.try_into()
    }

    pub fn apply(&self) -> Result<(), Box<dyn Error>> {
        // for entry in fs::read_dir(&self.root)? {
        //     let entry = entry?;
        //     let path = entry.path();
    
        //     let metadata = fs::metadata(&path)?;
        //     let last_modified = metadata.modified()?.elapsed()?.as_secs();
    
        //     // if last_modified < 24 * 3600 && metadata.is_file() {
        //     if metadata.is_file() {
        //         println!(
        //             "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
        //             last_modified,
        //             metadata.permissions().readonly(),
        //             metadata.len(),
        //             path.file_name().ok_or("No filename")?
        //         );
        //     }
        // }

        let files = GlobWalkerBuilder::from_patterns(
                &self.root,
                &["**/*", "!over.*"],
            )
            .sort_by(|a, b| a.path().cmp(b.path()))
            .build()?
            .into_iter()
            .filter_map(Result::ok);
    
        for file in files {
            println!("{:#?}", file);
        }
        
        Ok(())
    }
}
