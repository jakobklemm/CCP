//! Utility functions

use crate::Config;
use crate::DATABASE;
use polodb_core::bson::doc;

use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

pub fn ensure_configured() -> Result<()> {
    let col = DATABASE.collection::<Config>("config");
    let config = col.find_one(doc! {
        "_id": "CONFIG"
    })?;
    match config {
        Some(_c) => Ok(()),
        None => {
            let config = Config::default();
            let _ = col.insert_one(config);
            Ok(())
        }
    }
}

pub fn terminal_startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    Ok(())
}

pub fn terminal_shutdown() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
