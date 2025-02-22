use std::path::PathBuf;

use anyhow::Result;
use clap::{crate_name, Parser, Subcommand};

use crate::exec;

mod apply;
mod list;
mod show;
mod status;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    name = crate_name!(),
    long_about = None,
	// after_help = "over allows you to version your configuration files and workspaces settings",
)]
pub struct CLI {
    #[clap(
        long,
        short = 'H',
        global = true,
        required = false,
        env = "OVER_HOME",
        help = "Configuration and overlays root"
    )]
    home: PathBuf,

    #[clap(
        long,
        short = 'n',
        global = true,
        help = "Run without applying changes"
    )]
    dry_run: bool,

    #[clap(long, short, global = true, help = "Toggle debug traces")]
    debug: bool,

    #[clap(long, short, global = true, help = "Toggle verbose output")]
    verbose: bool,

    #[clap(subcommand)]
    cmd: Option<Commands>,
}

impl From<&CLI> for exec::Context {
    fn from(val: &CLI) -> Self {
        exec::Context {
            dry_run: val.dry_run,
            debug: val.debug,
            verbose: val.verbose,
            ..Default::default()
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "list", about = "List known overlays", alias = "ls")]
    List(list::Params),

    #[clap(name = "show", about = "Display details about an overlay")]
    Show(show::Params),

    #[clap(name = "apply", about = "Apply a given overlay")]
    Apply(apply::Params),

    #[clap(
        name = "status",
        about = "Get the current repository/directory overlays status"
    )]
    Status,
}

pub async fn main() -> Result<()> {
    let args = CLI::parse();
    match args.cmd {
        Some(Commands::List(ref opt)) => {
            list::execute(&args, opt).await?;
        }
        Some(Commands::Apply(ref opt)) => {
            apply::execute(&args, opt).await?;
        }
        Some(Commands::Show(ref opt)) => {
            show::execute(&args, opt).await?;
        }
        Some(Commands::Status) => {
            status::execute(&args).await?;
        }
        None => {
            println!("args: {:?}", args);
        }
    }
    Ok(())
}
