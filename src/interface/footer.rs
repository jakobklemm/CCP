//! # Footer

use crate::interface::Render;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, prelude::Frame};

#[derive(Clone, Debug)]
pub struct Footer {}

impl Render for Footer {
    fn render(&self, f: &mut Frame, area: Rect) {}
    fn input(&mut self, key: KeyEvent) {}
}
