//! Application

use crate::database::Database;
use crate::entry::Entry;
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
