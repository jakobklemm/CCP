//! Utility functions

use crate::DATABASE;
use crate::ROOT;
use polodb_core::bson::doc;

use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use std::fs;
use std::io::stdout;
use tantivy::Index;

pub fn ensure_configured() -> Result<()> {
    let path1 = format!("{}/store/", ROOT.as_str());
    let path2 = format!("{}/ingest/", ROOT.as_str());
    let path3 = format!("{}/data/", ROOT.as_str());
    let path4 = format!("{}/meta/", ROOT.as_str());

    let _ = fs::create_dir_all(path1);
    let _ = fs::create_dir_all(path2);
    let _ = fs::create_dir_all(path3);
    let _ = fs::create_dir_all(path4);

    Ok(())
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

pub fn center(width: u16, height: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - width) / 2),
            Constraint::Percentage(width),
            Constraint::Percentage((100 - width) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - height) / 2),
            Constraint::Percentage(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(layout[1])[1]
}
