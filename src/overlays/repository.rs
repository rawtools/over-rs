use std::error::Error;
use std::path::PathBuf;

use config::ConfigError;
use globwalk::GlobWalkerBuilder;

use super::{BASENAME, EXTENSIONS};
use super::overlay::Overlay;

/// Manage all overlays
#[derive(Debug)]
pub struct Repository {
    /// Repository root directory
    root: PathBuf,
}

// impl std::fmt::Display for Repository {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         std::fmt::Display::fmt(&self.root.display(), f)
//     }
// }

impl Repository {

    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Returns a list of all overlays in the repository
    pub fn overlays(&self) -> Result<Vec<Overlay>, Box<dyn Error>> {
        let mut out: Vec<Overlay> = Vec::new();
        let pattern = format!("**/{}.{{{}}}", BASENAME, EXTENSIONS.join(","));
        let overlay_files = GlobWalkerBuilder::new(&self.root, pattern)
            .build()?
            .into_iter()
            .filter_map(Result::ok);

        for file in overlay_files {
            let name = file.path().parent().unwrap().strip_prefix(&self.root).unwrap().to_str().unwrap();
            let root = PathBuf::from(file.path().parent().unwrap());
            let overlay = Overlay::new(name,  &root)?;
            if !overlay.parent {
                out.push(overlay);
            }
        }
        out.sort_by(|a, b| a.root.cmp(&b.root));
        Ok(out)
    }

    /// Get a repository by its name/relative path
    pub fn get(&self, name: &str) -> Result<Overlay, ConfigError> {
        let root = self.root.join(name);
        let overlay = Overlay::new(name, &root)?;
        Ok(overlay)
    }
}
