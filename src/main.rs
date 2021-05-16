mod cli;
mod portals;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use portals::Portal;
use std::path::PathBuf;
use structopt::StructOpt;

fn find_default_portal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".portal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        portal_file,
    } = CommandLineArgs::from_args();

    let portal_file = portal_file
        .or_else(find_default_portal_file)
        .ok_or(anyhow!("Failed to find portal file"))?;

    match action {
        Add { label, path } => portals::add_portal(portal_file, Portal::new(label, path)),
        List => portals::list_portals(portal_file),
        Remove { label } => portals::remove_portal(portal_file, label),
        Go { label } => portals::go_portal(portal_file, &label),
    }?;
    Ok(())
}
