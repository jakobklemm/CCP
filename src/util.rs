//! Utility functions

use crate::ROOT;

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

pub fn ensure_configured() -> Result<()> {
    let path1 = format!("{}/store/", ROOT.as_str());
    let path2 = format!("{}/ingest/", ROOT.as_str());
    let path3 = format!("{}/data/", ROOT.as_str());
    let path4 = format!("{}/meta/", ROOT.as_str());
    let path5 = format!("{}/temp/", ROOT.as_str());
    let path6 = format!("{}/source/", ROOT.as_str());
    let path7 = format!("{}/subs/", ROOT.as_str());

    let _ = fs::create_dir_all(path1);
    let _ = fs::create_dir_all(path2);
    let _ = fs::create_dir_all(path3);
    let _ = fs::create_dir_all(path4);
    let _ = fs::create_dir_all(path5);
    let _ = fs::create_dir_all(path6);
    let _ = fs::create_dir_all(path7);

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
