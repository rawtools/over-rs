use std::sync::{Arc, RwLock};

use indicatif::MultiProgress;
use serde::Serialize;

use crate::overlays::{Overlay, Repository};

#[derive(Debug, Default, Serialize)]
pub struct Context {
    /// Run without applying changes
    pub dry_run: bool,

    /// Toggle debug traces,
    pub debug: bool,

    /// Toggle verbose output
    pub verbose: bool,

    pub repository: Repository,

    pub overlay: Option<Overlay>,

    pub state: RwLock<State>,
    // pub progress: Option<MultiProgress>,
}

#[derive(Debug, Default, Serialize)]
pub struct State {
    #[serde(skip)]
    pub progress: Option<MultiProgress>,
}

// #[derive(Debug, Default, Serialize)]
// pub struct Channel<T> {
//     // pub tx: Sender<T>,
//     // pub rx: Receiver<T>,
// }

// // impl Default for Context {
// //     fn default() -> Self {
// //         Self { dry_run: Default::default(), debug: Default::default(), verbose: Default::default(), repository: Default::default(), overlay: Default::default() }
// //     }
// // }

impl Context {
    pub fn new(
        dry_run: bool,
        debug: bool,
        verbose: bool,
        repository: Repository,
        overlay: Option<Overlay>,
    ) -> Arc<Self> {
        Arc::new(Self {
            dry_run,
            debug,
            verbose,
            repository,
            overlay,
            state: RwLock::new(State::default()),
            // progress: None,
        })
    }
}

pub type Ctx = Arc<Context>;
