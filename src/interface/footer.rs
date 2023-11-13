//! # Footer

use crate::interface::Render;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Alignment, Rect},
    prelude::Frame,
    style::{Color, Style},
    widgets::{block::Title, Block, BorderType, Borders, Paragraph},
};

#[derive(Clone, Debug)]
pub struct Footer {}

impl Render for Footer {
    fn render(&self, f: &mut Frame, area: Rect) {
        let outer = Block::default()
            .title(Title::from(" Footer ").alignment(Alignment::Left))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightRed));

        let cont = format!(
            " Use <C-h> and <C-l> to switch between tabs, use <tab> and <TAB> to navigate inside."
        );
        let p = Paragraph::new(cont)
            .block(outer)
            .style(Style::default().fg(Color::Gray));

        f.render_widget(p, area);
    }
    fn input(&mut self, key: KeyEvent) {}
}

impl Default for Footer {
    fn default() -> Self {
        Self {}
    }
}
