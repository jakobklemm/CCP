//! # Settings

use ratatui::widgets::Paragraph;

use crate::interface::Render;

#[derive(Clone, Debug, Default)]
pub struct Settings {}

impl Render for Settings {
    fn render(&mut self, f: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        f.render_widget(Paragraph::new("settings"), area)
    }

    fn input(&mut self, key: crossterm::event::KeyEvent) {}
}
