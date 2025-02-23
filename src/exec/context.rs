use std::sync::Arc;

use indicatif::{MultiProgress, ProgressBar};
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

    #[serde(skip)]
    pub progress: Option<Progress>,
}

// Store the current progress bar
#[derive(Debug, Clone)]
pub enum Progress {
    Progress(ProgressBar),
    MultiProgress(MultiProgress),
}

impl Progress {
    pub fn try_progress(&self) -> Option<&ProgressBar> {
        match self {
            Progress::Progress(p) => Some(p),
            _ => None,
        }
    }

    pub fn try_multiprogress(&self) -> Option<&MultiProgress> {
        match self {
            Progress::MultiProgress(p) => Some(p),
            _ => None,
        }
    }
}

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
            progress: None,
        })
    }

    pub fn with_overlay(&self, overlay: Overlay) -> Arc<Self> {
        Arc::new(Self {
            dry_run: self.dry_run,
            debug: self.debug,
            verbose: self.verbose,
            repository: self.repository.clone(),
            overlay: Some(overlay),
            progress: self.progress.clone(),
        })
    }

    pub fn with_progress(&self, progress: ProgressBar) -> Arc<Self> {
        Arc::new(Self {
            dry_run: self.dry_run,
            debug: self.debug,
            verbose: self.verbose,
            repository: self.repository.clone(),
            overlay: self.overlay.clone(),
            progress: Some(Progress::Progress(progress)),
        })
    }

    pub fn with_multiprogress(&self, progress: MultiProgress) -> Arc<Self> {
        Arc::new(Self {
            dry_run: self.dry_run,
            debug: self.debug,
            verbose: self.verbose,
            repository: self.repository.clone(),
            overlay: self.overlay.clone(),
            progress: Some(Progress::MultiProgress(progress)),
        })
    }

    pub fn try_progress(&self) -> Option<&ProgressBar> {
        self.progress.as_ref().and_then(|p| p.try_progress())
    }

    pub fn try_multiprogress(&self) -> Option<&MultiProgress> {
        self.progress.as_ref().and_then(|p| p.try_multiprogress())
    }
}

pub type Ctx = Arc<Context>;
