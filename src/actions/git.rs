use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;


use gix;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::future::join_all;
use git2::Progress;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use tokio::{
    spawn,
    sync::mpsc::{self, Sender},
    task::spawn_blocking,
};

use crate::{
    exec::{Action, Ctx},
    ui::{self, emojis, style},
};

pub async fn clone_repositories(ctx: Ctx, to: &Path) -> Result<()> {
    if let Some(overlay) = &ctx.overlay {
        if let Some(git_repos) = &overlay.git {
            ui::info(format!(
                "{} {}",
                emojis::THREAD,
                style::white("Cloning repositories"),
            ))?;
            let progress = MultiProgress::new();

            {
                let mut state = ctx.state.write().unwrap();
                state.progress = Some(progress);
            }
            let _clones = join_all(git_repos.iter().map(|(path, url)| {
                let target = to.join(path);
                let url = url.to_string();
                let ctx = ctx.clone();
                spawn(async move {
                    let action = EnsureGitRepository::new(target, url.to_string());
                    action.execute(ctx).await
                })
            }))
            .await;
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

    fn short_name(&self) -> &'static str {
        let name = String::from(
            self.remote
                .split('/')
                .last()
                .unwrap()
                .trim_end_matches(".git"),
        );
        Box::leak(name.into_boxed_str())
    }
}

impl fmt::Display for EnsureGitRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.path.display(), self.remote)
    }
}

#[async_trait]
impl Action for EnsureGitRepository {
    async fn execute(&self, ctx: Ctx) -> Result<()> {
        // let overlay = ctx.overlay.as_ref().unwrap();
        // let target = overlay.resolve_target(ctx.as_ref())?;
        // let relpath = self.path.strip_prefix(target.as_path())?;
        // if ctx.verbose || ctx.dry_run {
        //     ui::info(format!(
        //         "{} {} {} {} {}",
        //         emojis::THREAD,
        //         "clone:".fg::<White>(),
        //         self.remote,
        //         "->".fg::<White>(),
        //         self.path.display(),
        //     ))?;
        // }

        let pb = ctx.state.read().unwrap().progress.as_ref().map(|progress| 
                progress
                    .add(ProgressBar::new(100))
                    .with_style(CLONE_PROGRESS_STYLE.clone())
                    .with_prefix(self.short_name())
        );

        if self.path.exists() {
            if let Some(p) = pb.as_ref() {
                // println!("p: {:#?}", p);
                // p.with_style(CLONE_ERROR_STYLE.clone());
                if ctx.verbose {
                    p.println("Repository exists")
                }
                p.finish_and_clear();
            } else if ctx.verbose {
                println!("Repository exists");
            }
            // Repository::open(self.path.as_path())?
        } else if !ctx.dry_run {
                let mut state = CloneState::default();
                let url = self.remote.clone();
                let into = self.path.clone();
                let (tx, mut rx) = mpsc::channel(100);
                let tx = Arc::new(tx);
                let task = spawn_blocking(move || clone(&url, &into, &tx));

                while let Some(msg) = rx.recv().await {
                    match msg {
                        CloneMessage::Progress(pr) => state.progress = pr,
                        CloneMessage::Stats(s) => state.stats = s,
                    }
                    if let Some(p) = pb.as_ref() {
                        state.update_bar(p);
                    }
                }

                if let Err(e) = task.await? {
                    if let Some(p) = pb.as_ref() {
                        p.println(format!("{} {}", emojis::CROSSMARK, e));
                        p.abandon_with_message(format!("{} Failed", emojis::CROSSMARK));
                    }
                    return Err(anyhow!(e));
                } else if let Some(p) = pb.as_ref() {
                    p.finish_and_clear();
                }
            
        };

        Ok(())
    }
}

static CLONE_PROGRESS_STYLE: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle
        ::with_template(
            "{spinner:.cyan} {prefix} [{bar:.green/yellow}] {msg}",
        ).unwrap()
        .tick_chars(style::TICK_CHARS_BRAILLE_4_6_DOWN.as_str())
        .progress_chars(style::THIN_PROGRESS.as_str())
});


// fn clone(url: &str, dst: &Path, progress: &Sender<CloneMessage>) -> Result<Repository> {
//     let mut cb = git2::RemoteCallbacks::new();
//     let git_config = git2::Config::open_default().unwrap();
//
//     // Credentials management
//     let mut ch = CredentialHandler::new(git_config);
//     cb.credentials(move |url, username, allowed| ch.try_next_credential(url, username, allowed));
//     cb.transfer_progress(|stats| {
//         let stats = CloneStats::from(stats);
//         progress.blocking_send(CloneMessage::Stats(stats)).unwrap();
//         true
//     });
//
//     let mut co = git2::build::CheckoutBuilder::new();
//     co.progress(|path, cur, total| {
//         let prog = CloneProgress {
//             path: path.map(|p| p.to_path_buf()),
//             current: cur,
//             total,
//         };
//         progress.blocking_send(CloneMessage::Progress(prog)).unwrap();
//     });
//
//     // clone a repository
//     let mut fo = git2::FetchOptions::new();
//     fo.remote_callbacks(cb)
//         .download_tags(git2::AutotagOption::All)
//         .update_fetchhead(true);
//     // std::fs::create_dir_all(&dst.as_ref()).unwrap();
//     
//     let repo = git2::build::RepoBuilder::new()
//         .fetch_options(fo)
//         .with_checkout(co)
//         .clone(url, dst)?;
//
//     Ok(repo)
// }


fn clone(url: &str, dst: &Path, progress: &Sender<CloneMessage>) -> Result<gix::Repository> {
    // TODO: progress
    // TODO: ssh auth

    std::fs::create_dir_all(dst)?;
    // println!("url: {:#?}", git::Url::from_bytes(url.try_into()?));
    // let mut prepare = git::prepare_clone("https://github.com/noirbizarre/slides-paris.py/", dst)?;
    let mut prepare = gix::prepare_clone(url, dst)?;
    // let mut prepare = git::prepare_clone(url.try_into()?, dst)?;
    println!("prepare ok");
    // println!("{:#?}", prepare);
    
    // let mut prepare = git::clone::PrepareFetch::new(
    //     url,
    //     dst,
    //     git::create::Kind::WithWorktree,
    //     git::create::Options::default(),
    //     git::open::Options::default(),
    // ).unwrap();

    let (mut checkout, _out) = prepare.fetch_then_checkout(gix::progress::Discard, &std::sync::atomic::AtomicBool::default())?;
    println!("checkout ok");
    let (repo, _) = checkout.main_worktree(gix::progress::Discard, &std::sync::atomic::AtomicBool::default())?;
    println!("checkout main ok");

    // let index = repo.index()?;
    // assert_eq!(index.entries().len(), 1, "All entries are known as per HEAD tree");

    // let work_dir = repo.work_dir().expect("non-bare");
    // for entry in index.entries() {
    //     let entry_path = work_dir.join(git_path::from_bstr(entry.path(&index)));
    //     assert!(entry_path.is_file(), "{:?} not found on disk", entry_path)
    // }

    Ok(repo)
}

#[derive(Debug, Default)]
struct CloneStats {
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
    fn update_bar(&self, bar: &ProgressBar) -> Result<()> {
        let stats = &self.stats;
        let network_pct = (100 * stats.received_objects) / stats.total_objects;
        let index_pct = (100 * stats.indexed_objects) / stats.total_objects;
        let co_pct = if self.progress.total > 0 {
            (100 * self.progress.current) / self.progress.total
        } else {
            0
        };
        bar.set_length(u64::try_from(stats.total_objects)?);
        bar.set_position(u64::try_from(stats.indexed_objects)?);
        let kbytes = stats.received_bytes / 1024;
        if stats.received_objects == stats.total_objects {
            bar.set_message(format!(
                "Resolving deltas {}/{}\r",
                stats.indexed_deltas, stats.total_deltas
            ));
        } else {
            bar.set_message(format!(
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
                self.progress
                    .path
                    .as_ref()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default()
            ));
        }
        Ok(())
    }
}
