use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write portal to the portal file.
    Add {
        /// The portal target path.
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the portal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all portals in the Portal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "Portal",
about = "Change Directory Helper."
)]
pub struct CommandLineArgs{
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different portal file.
    #[structopt(parse(from_os_str), short, long)]
    pub portal_file: Option<PathBuf>,
}