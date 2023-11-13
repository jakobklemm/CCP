//! # Header

use crate::interface::{Render, STATES};
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Direction, Layout, Rect},
    Frame,
};

#[derive(Debug, Clone)]
pub struct Header {
    selected: i32,
}

impl Render for Header {
    fn render(&self, f: &mut Frame, area: Rect) {}
    /// TODO: Handle C-h and C-l for next tab.
    fn input(&mut self, key: KeyEvent) {}
}

impl Header {
    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % STATES;
    }

    pub fn previous(&mut self) {
        self.selected = (STATES + self.selected - 1) % STATES;
    }
}
