use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Result, Error, ErrorKind, Seek, SeekFrom};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Portal {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Portal {
    pub fn new(text: String) -> Portal {
        let created_at: DateTime<Utc> = Utc::now();
        Portal { text, created_at }
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

pub fn remove_portal(portal_path: PathBuf, position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(portal_path)?;

    let mut portals = collect_portals(&file)?;

    if position == 0 || position > portals.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Portal ID"));
    }
    portals.remove(position - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &portals)?;
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

impl fmt::Display for Portal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}