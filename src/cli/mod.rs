use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod apply;
mod list;
mod status;

#[derive(Parser, Debug)]
#[structopt( 
    name = "over", 
	about = "git-based overlays",
	after_help = "over allows you to version your configuration files and workspaces settings",
)]
pub struct CLI {
    #[clap(
        long, short = 'H',
        global = true,
        required = false,
        env = "OVER_HOME",
        help = "Configuration and overlays root",
        parse(from_os_str),
    )]
    home: PathBuf,

    #[clap(
        long, short = 'n',
        help = "Run without applying changes",
    )]
    dry_run: bool,

    #[clap(
        long, short,
        help = "Toggle debug traces",
    )]
    debug: bool,

    #[clap(
        long, short,
        help = "Toggle verbose output",
    )]
    verbose: bool,

    #[clap(subcommand)]
    cmd: Option<Commands>
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "apply", about = "Apply a given overlay")]
    Apply(apply::Params),
    
    #[clap(name = "list", about = "List known overlays", alias = "ls")]
    List(list::Params),
    
    #[clap(name = "status", about = "Get the current repository/directory overlays status")]
    Status,
}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLI::parse();
    match args.cmd {
        Some(Commands::Apply(ref opt)) => {
            apply::execute(&args, opt)?;
        }
        Some(Commands::List(ref opt)) => {
            list::execute(&args, opt)?;
        }
        Some(Commands::Status) => {
            status::execute(&args);
        }
        None => {
            println!("args: {:?}", args);
        }
    }
    Ok(())
}
