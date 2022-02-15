use std::path::PathBuf;

use walkdir::WalkDir;
use globset::GlobBuilder;

use crate::Expect;

use super::pattern;
use super::overlay::Overlay;


/// Manage all overlays
#[derive(Debug)]
pub struct Repository {
    /// Repository root directory
    pub root: PathBuf,
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
    pub fn overlays(&self) -> Expect<Vec<Overlay>> {
        let glob = GlobBuilder::new(&pattern()).literal_separator(true).build()?.compile_matcher();

        let mut dirs: Vec<PathBuf> = WalkDir::new(&self.root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| glob.is_match(e.path().strip_prefix(&self.root).ok().unwrap()))
            .map(|e| e.path().parent().unwrap().to_path_buf())
            .collect();

        dirs.sort();

        Ok(
            dirs.iter().enumerate()
            .filter_map(|(idx, dir)| {
                match dirs.get(idx + 1) {
                    Some(next) if next.starts_with(&dir) => None,
                    _ => Some(Overlay::new(self, &dir).expect("failed")),
                }
            })
            .collect()
        )

    }

    /// Get a repository by its name/relative path
    pub fn get(&self, name: &str) -> Expect<Overlay> {
        let root = self.root.join(name);
        let overlay = Overlay::new(self, &root)?;
        Ok(overlay)
    }
}
