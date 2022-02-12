use std::path::{Path, PathBuf};

use glob::glob;

use super::EXTENSIONS;
use super::overlay::Overlay;

#[derive(Debug)]
pub struct Repository {
    root: PathBuf,
}

impl std::fmt::Display for Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.root.display(), f)
    }
}

impl Repository {

    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn overlays(&self) -> Vec<Overlay> {
        let mut out: Vec<Overlay> = Vec::new();
        let pattern = format!("{}/**/over.*", self.root.display());
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if EXTENSIONS.contains(&path.extension().unwrap().to_str().unwrap()) {
                        let name = Path::new(&path).parent().unwrap().strip_prefix(&self.root).unwrap().to_str().unwrap();
                        let overlay = Overlay::load(String::from(name),  &path);
                        if overlay.config.overlay {
                            out.push(overlay);
                        }
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
        return out
    }
}
