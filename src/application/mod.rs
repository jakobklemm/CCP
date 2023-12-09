//! Application

use crate::interface::Render;
use crate::interface::{Interface, State};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::Style,
    widgets::{Block, Borders, List, ListState},
};
use std::time::Instant;
use tui_textarea::TextArea;

pub mod actions;
pub mod entry;
mod filelist;
pub mod id;
pub mod indexed;
pub mod job;
pub mod metadata;
mod parse_date;
pub mod processor;
pub mod status;
pub mod tag;
mod timestamp;

pub use entry::Entry;
pub use filelist::load;
pub use metadata::Metadata;
pub use parse_date::parse;

#[derive(Debug, Default)]
pub struct App {
    pub ui: Interface,
    quit: bool,
}

impl App {
    // Tick event of terminal
    pub fn tick(&mut self) {
        self.ui.tick()
    }

    pub fn input(&mut self, key: KeyEvent) {
        self.ui.input(key);
    }

    pub fn next(&mut self) {
        self.ui.next();
    }

    pub fn previous(&mut self) {
        self.ui.previous();
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    // Quit event
    pub fn quit(&mut self) {
        self.quit = true;
    }
}
