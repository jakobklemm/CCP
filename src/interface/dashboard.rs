//! # Dashboard

use crate::interface::Render;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::{Alignment, Direction, Frame},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::fmt::Display;
use tui_big_text::BigTextBuilder;

#[derive(Debug, Clone)]
pub struct Dashboard {
    count: u64,
    tags: u64,
    size: f64,
}

impl Default for Dashboard {
    fn default() -> Self {
        // TODO: Fetch data from DB
        Self {
            count: Default::default(),
            tags: Default::default(),
            size: Default::default(),
        }
    }
}

impl Render for Dashboard {
    fn render(&self, f: &mut Frame, area: Rect) {
        let thirds = Layout::default()
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .direction(Direction::Horizontal)
            .split(area);

        Self::render_overview(&self, f, thirds[0]);
        Self::render_center(&self, f, thirds[1]);
        Self::render_tags(&self, f, thirds[2]);
    }

    /// Dashboard isn't interactive
    fn input(&mut self, key: KeyEvent) {}
}

impl Dashboard {
    fn render_tags(&self, f: &mut Frame, area: Rect) {
        // TODO: Fetch tags
        f.render_widget(
            Paragraph::new("TODO".to_string()).block(Block::default().borders(Borders::ALL)),
            area,
        );
    }

    fn render_center(&self, f: &mut Frame, area: Rect) {
        let around = Block::default()
            .title(" DASHBOARD ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double);

        let cont = format!("\n \nVersion: 0.1.0\n");

        let para = Paragraph::new(cont)
            .style(Style::default().fg(Color::Yellow))
            .block(around);

        f.render_widget(para, area);
    }

    fn render_overview(&self, f: &mut Frame, area: Rect) {
        let thirds = Layout::default()
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .direction(Direction::Vertical)
            .split(area);

        Self::render_large(f, self.count, Color::Red, thirds[0]);
        Self::render_large(f, self.tags, Color::Green, thirds[1]);
        Self::render_large(f, self.size, Color::Blue, thirds[2]);
    }

    fn render_large<T: Display>(f: &mut Frame, value: T, color: Color, area: Rect) {
        let line = vec![format!("{}", value).red().into()];

        let top = BigTextBuilder::default()
            .style(Style::default().fg(color))
            .lines(line)
            .build()
            .unwrap();

        f.render_widget(top, area);
    }
}
