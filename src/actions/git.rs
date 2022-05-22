use std::path::{PathBuf, Path};
use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::future::join_all;
use git2_credentials::CredentialHandler;
use tokio::{spawn, task::spawn_blocking, sync::mpsc::{self, Sender}};
use git2::{Repository, Progress};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use owo_colors::{OwoColorize, colors::*};

use crate::{
    exec::{Action, Ctx}, 
    ui::{self, emojis, style}
};

pub async fn clone_repositories(ctx: Ctx, to: &Path) -> Result<()> {
    if let Some(overlay) = &ctx.overlay {
        if let Some(git_repos) = &overlay.git {
            ui::info(format!("{} {}",
                emojis::THREAD,
                style::WHITE.apply_to("Cloning repositories"), 
            ))?;
            let progress = MultiProgress::new();
    
            {
                let mut state = ctx.state.write().unwrap();
                state.progress = Some(progress);
            }
            let clones = join_all(git_repos.iter().map(|(path, url)| {
                let target = to.join(path);
                let url = url.to_string();
                let ctx = ctx.clone();
                spawn(async move {
                    let action = EnsureGitRepository::new(target, url.to_string());
                    action.execute(ctx).await
                })
            }));
            
            let handle = spawn_blocking(move || {
                let state = ctx.state.read().unwrap();
                state.progress.as_ref().unwrap().join();
            });
            // let m_handle = spawn_blocking(move || m.join().unwrap());

            clones.await;
            handle.await?;

            // let state = ctx.state.read().unwrap();
            // println!("State {:#?} -> {:#?}", state, state.progress.as_ref());
            // state.progress.as_ref().unwrap().join()?;
            // println!("results: {:#?}", results);
        };
    };
    Ok(())
}


pub struct EnsureGitRepository {
    pub path: PathBuf,
    pub remote: String,
}

impl EnsureGitRepository {
    pub fn new(path: PathBuf, remote: String) -> Self {
        Self { path, remote }
    }
} 

// impl fmt::Display for EnsureGitRepository {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} -> {}", self.source.display(), self.target.display())
//     }
// }


#[async_trait]
impl Action for EnsureGitRepository {
    async fn execute(&self, ctx: Ctx) -> Result<()> {
        if ctx.verbose || ctx.dry_run {
            ui::info(  
                format!("{} {} {} {} {}",
                    emojis::THREAD,
                    "clone:".fg::<White>(), 
                    self.remote,
                    "->".fg::<White>(), 
                    self.path.display(),
                )
            )?;
        }
        // let state = ctx.state.read().unwrap();
        // println!("State {:#?} -> {:#?}", state, state.progress.as_ref());
        let pb = match ctx.state.read().unwrap().progress.as_ref() {
            Some(progress) => Some(
                progress.add(ProgressBar::new(100))
                    .with_style(CLONE_PROGRESS_STYLE.clone())
                    .with_message("Clone")
            ),
            _ => None,
        };

        if self.path.exists() {
            if let Some(p) = pb.as_ref() {
                // println!("p: {:#?}", p);
                // p.with_style(CLONE_ERROR_STYLE.clone());
                p.finish_with_message("Repository exists");
            } else if ctx.verbose {
                println!("Repository exists");
            }
            // Repository::open(self.path.as_path())?
        } else {
            if !ctx.dry_run {
                let mut state = CloneState::default();
                let url = self.remote.clone();
                let into = self.path.clone();
                let (tx, mut rx) = mpsc::channel(100);
                let tx = Arc::new(tx);
                let task = spawn_blocking(move || clone(&url, &into, &tx));

                while let Some(msg) = rx.recv().await {
                    // println!("recv: {:#?}", msg);
                    match msg {
                        CloneMessage::Progress(pr) => state.progress = pr,
                        CloneMessage::Stats(s) => state.stats = s,
                    }
                    // println!("state: {:#?}", state)
                    if let Some(p) = pb.as_ref() {
                        state.update_bar(p);
                    }
                }

                match task.await? {
                    Ok(_) => if let Some(p) = pb.as_ref() {
                        // p.println(e.to_string());
                        p.finish_with_message("Cloned");
                        // repo
                    }
                    Err(e) => {
                        if let Some(p) = pb.as_ref() {
                            // p.println(e.to_string());
                            p.abandon_with_message(e.to_string());
                        }
                        return Err(anyhow!(e))
                    },
                };
            }
        };

        Ok(())
    }

}




lazy_static! {

    // static ref CLONE_PROGRESS_STYLE: ProgressStyle = ProgressStyle::default_bar()
    //     .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {wide_msg:.red}{bytes}/{total_bytes} ({eta})")
    //     .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    static ref CLONE_PROGRESS_STYLE: ProgressStyle = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {msg}")
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

        static ref CLONE_ERROR_STYLE: ProgressStyle = ProgressStyle::default_bar()
        .template("{spinner:.green} {wide_msg:.red}");
}


fn clone(url: &str, dst: &Path, progress: &Sender<CloneMessage>) -> Result<Repository> {
    let mut cb = git2::RemoteCallbacks::new();
    let git_config = git2::Config::open_default().unwrap();

    // Credentials management
    let mut ch = CredentialHandler::new(git_config);
    cb.credentials(move |url, username, allowed| ch.try_next_credential(url, username, allowed));
    let mut co = git2::build::CheckoutBuilder::new();
    cb.transfer_progress(|stats| {
        let cs = CloneStats::from(stats);
        progress.blocking_send(CloneMessage::Stats(cs)).unwrap();
        true
    });
    co.progress(|path, cur, total| {
        let cp = CloneProgress {
            path: path.map(|p| p.to_path_buf()),
            current: cur,
            total,
        };
        progress.blocking_send(CloneMessage::Progress(cp)).unwrap();
    });

    // clone a repository
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb)
        .download_tags(git2::AutotagOption::All)
        .update_fetchhead(true);
    // std::fs::create_dir_all(&dst.as_ref()).unwrap();
    let repo = git2::build::RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(url, dst)?;

    Ok(repo)
}


#[derive(Debug, Default)]
struct CloneStats {
    // progress: Option<Progress<'static>>,
    total_objects: usize,
    indexed_objects: usize,
    received_objects: usize,
    local_objects: usize,
    total_deltas: usize,
    indexed_deltas: usize,
    received_bytes: usize,
}

unsafe impl Send for CloneStats {}

impl CloneStats {
    fn from(stats: Progress) -> Self {
        Self { 
            total_objects: stats.total_objects(),
            indexed_objects: stats.indexed_objects(),
            received_objects: stats.received_objects(),
            local_objects: stats.local_objects(),
            total_deltas: stats.total_deltas(),
            indexed_deltas: stats.indexed_deltas(),
            received_bytes: stats.received_bytes(),
        }
    }
}

#[derive(Debug, Default)]
struct CloneProgress {
    total: usize,
    current: usize,
    path: Option<PathBuf>,
}

#[derive(Debug)]
enum CloneMessage {
    Stats(CloneStats),
    Progress(CloneProgress),
}

unsafe impl Send for CloneProgress {}

#[derive(Debug, Default)]
struct CloneState {
    stats: CloneStats,
    progress: CloneProgress,
}


impl CloneState {
    fn update_bar(&self, bar: &ProgressBar) {
        let stats = &self.stats;
        let network_pct = (100 * stats.received_objects) / stats.total_objects;
        let index_pct = (100 * stats.indexed_objects) / stats.total_objects;
        let co_pct = if self.progress.total > 0 {
            (100 * self.progress.current) / self.progress.total
        } else {
            0
        };
        let kbytes = stats.received_bytes / 1024;
        if stats.received_objects == stats.total_objects {
            // if !state.newline {
            //     println!();
            //     state.newline = true;
            // }
            bar.set_message(
                format!(
                    "Resolving deltas {}/{}\r",
                    stats.indexed_deltas,
                    stats.total_deltas
                )
            );
        } else {
            bar.set_message(
                format!(
                    "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
                    /  chk {:3}% ({:4}/{:4}) {}\r",
                    network_pct,
                    kbytes,
                    stats.received_objects,
                    stats.total_objects,
                    index_pct,
                    stats.indexed_objects,
                    stats.total_objects,
                    co_pct,
                    self.progress.current,
                    self.progress.total,
                    self.progress.path
                        .as_ref()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_default()
                )
            );
        }
    }
}
