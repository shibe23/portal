use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Portal {
    pub label: String,
    pub path: String,
}

impl Portal {
    pub fn new(label: String, path: String) -> Portal {
        Portal { label, path }
    }
}

fn collect_portals(mut file: &File) -> Result<Vec<Portal>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let portals = match serde_json::from_reader(file) {
        Ok(portals) => portals,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(portals)
}

pub fn add_portal(portal_path: PathBuf, portal: Portal) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(portal_path)?;
    let mut portals = collect_portals(&file)?;
    portals.push(portal);
    serde_json::to_writer(file, &portals)?;
    Ok(())
}

pub fn remove_portal(portal_path: PathBuf, label: String) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(portal_path)?;

    let mut portals = collect_portals(&file)?;

    if portals.is_empty() {
        println!("Portal list is empty!");
    } else {
        let index = find_matched_portal_index(&portals, &label);
        match index {
            Some(x) => {
                println!("{}", &portals[x].path);
                &portals.remove(x);
                file.set_len(0)?;
                serde_json::to_writer(file, &portals)?;
            }
            None => {
                println!("Cannot find this label.");
            }
        }
    }
    Ok(())
}

pub fn list_portals(portal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(portal_path)?;
    let portals = collect_portals(&file)?;

    if portals.is_empty() {
        println!("Portal list is empty!");
    } else {
        let mut order: u32 = 1;
        for portal in portals {
            println!("{}: {}", order, portal);
            order += 1;
        }
    }

    Ok(())
}

pub fn go_portal(portal_path: PathBuf, label: &String) -> Result<()> {
    let file = OpenOptions::new().read(true).open(portal_path)?;
    let portals = collect_portals(&file)?;

    if portals.is_empty() {
        println!("Portal list is empty!");
    } else {
        let index = find_matched_portal_index(&portals, label);
        match index {
            Some(x) => {
                println!("{}", &portals[x].path);
            }
            None => {
                println!("Cannot find this label.");
            }
        }
    }
    Ok(())
}

fn find_matched_portal_index(portals: &Vec<Portal>, label: &String) -> Option<usize> {
    portals.iter().position(|x| &x.label == label)
}

impl fmt::Display for Portal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<50} [{}]", self.label, self.path)
    }
}
