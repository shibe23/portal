mod cli;
mod portals;

use anyhow::anyhow;
use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use portals::Portal;
use std::path::PathBuf;

fn find_default_portal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".portal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        portal_file
    } = CommandLineArgs::from_args();

    let portal_file = portal_file
        .or_else(find_default_portal_file)
        .ok_or(anyhow!("Failed to find portal file"))?;

    match action {
        Add {label, path} => portals::add_portal(portal_file, Portal::new(label, path)),
        List => portals::list_portals(portal_file),
        Done {position} => portals::remove_portal(portal_file, position),
        Go {label} => portals::go_portals(portal_file, &label)
    }?;
    Ok(())
}
