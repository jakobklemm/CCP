//! # Actions
//!
//! Shell processes to be run
//!

use std::process::{Command, Stdio};

use anyhow::Result;

use crate::ROOT;

use super::Entry;

pub fn open_vlc(entry: Entry) -> Result<()> {
    let _ = Command::new("vlc")
        .arg(entry.out_path()?)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(())
}

pub fn open_nautilus(entry: Entry) -> Result<()> {
    let _ = Command::new("nautilus")
        .arg("--select")
        .arg(entry.out_path()?)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(())
}
