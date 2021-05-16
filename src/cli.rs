use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write portal to the portal file.
    Add {
        /// The portal target path.
        #[structopt()]
        label: String,

        path: String,
    },
    /// Remove an entry from the portal file by label.
    Remove {
        #[structopt()]
        label: String,
    },
    /// Edit an entry from the portal file by label.
    Edit {
        #[structopt()]
        label: String,
        path: String,
    },
    Go {
        /// Change Directory from the label.
        #[structopt()]
        label: String,
    },

    /// List all portals in the Portal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Portal", about = "Change Directory Helper.")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different portal file.
    #[structopt(parse(from_os_str), short, long)]
    pub portal_file: Option<PathBuf>,
}
