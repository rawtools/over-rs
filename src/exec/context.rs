use crate::overlays::{Repository, Overlay};

#[derive(Debug, Default)]
pub struct Context {
    
    /// Run without applying changes
    pub dry_run: bool,
    
    /// Toggle debug traces,
    pub debug: bool,
    
    /// Toggle verbose output
    pub verbose: bool,
    
    pub repository: Option<Repository>,

    pub overlay: Option<Overlay>,
}

// impl Default for Context {
//     fn default() -> Self {
//         Self { dry_run: Default::default(), debug: Default::default(), verbose: Default::default(), repository: Default::default(), overlay: Default::default() }
//     }
// }

impl Context {
}
