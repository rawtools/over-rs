use std::path::PathBuf;

use super::config::Config;


#[derive(Debug)]
pub struct Overlay {
    pub name: String,
    pub root: PathBuf,
    pub config: Config,
}

impl Overlay {
    pub fn load(name: String, cfg: &PathBuf) -> Self {
        Self {
            name,
            root: PathBuf::from(cfg.parent().unwrap()),
            config: Config::load(cfg).unwrap(),
        }
    }
}
