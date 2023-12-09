//! # Dashboard

use crate::{
    application::{tag::Tag, Entry},
    interface::Render,
    DATABASE,
};
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
    size: u64,
}

impl Default for Dashboard {
    fn default() -> Self {
        let mut count = 0;
        let mut size = 0;
        let mut tags = 0;
        for entry in DATABASE.get_all::<Entry>().unwrap() {
            let entry = entry.unwrap();
            count += 1;
            size += entry.get_meta().size;
        }
        for _tag in DATABASE.get_all::<Tag>().unwrap() {
            tags += 1;
        }
        // TODO: Fetch data from DB
        Self { count, tags, size }
    }
}

impl Render for Dashboard {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let thirds = Layout::default()
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .direction(Direction::Horizontal)
            .split(area);

        Self::render_overview(self, f, thirds[0]);
        Self::render_center(self, f, thirds[1]);
        Self::render_tags(self, f, thirds[2]);
    }

    /// Dashboard isn't interactive
    fn input(&mut self, key: KeyEvent) {}
}

impl Dashboard {
    fn render_tags(&mut self, f: &mut Frame, area: Rect) {
        // TODO: Fetch tags
        f.render_widget(
            Paragraph::new("TODO".to_string()).block(Block::default().borders(Borders::ALL)),
            area,
        );
    }

    fn render_center(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(70), Constraint::Min(0)])
            .split(area);

        let around = Block::default()
            .title(" DASHBOARD ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);

        let cont = format!("\n \nVersion: 0.1.0\n");

        let para = Paragraph::new(cont)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(around.clone());

        let lower = Paragraph::new("TODO").block(around);

        f.render_widget(para, layout[0]);
        f.render_widget(lower, layout[1]);
    }

    fn render_overview(&mut self, f: &mut Frame, area: Rect) {
        let thirds = Layout::default()
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(1),
            ])
            .direction(Direction::Vertical)
            .split(area);

        Self::render_large(f, self.count, Color::Red, "Clips", thirds[0]);
        Self::render_large(f, self.tags, Color::Green, "Tags", thirds[1]);
        Self::render_large(
            f,
            (self.size as f64) / 1_000_000_000.0,
            Color::Blue,
            "Size",
            thirds[2],
        );
    }

    fn render_large<T: Display>(
        f: &mut Frame,
        value: T,
        color: Color,
        text: impl ToString,
        area: Rect,
    ) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(20), Constraint::Min(0)])
            .direction(Direction::Horizontal)
            .split(area);

        let p = Layout::default()
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(50),
                Constraint::Percentage(30),
            ])
            .direction(Direction::Vertical)
            .split(layout[1]);

        let mut line = vec![format!("{}", value).red().into()];

        if &text.to_string() == "Size" {
            line = vec![format!("{:.1} Gb", value).red().into()];
        }

        let top = BigTextBuilder::default()
            .style(Style::default().fg(color))
            .lines(line)
            .build()
            .unwrap();

        f.render_widget(top, p[1]);

        let bot = BigTextBuilder::default()
            .style(Style::default().fg(Color::Blue))
            .lines(vec![text.to_string().blue().into()])
            .build()
            .unwrap();

        f.render_widget(bot, p[2]);
    }
}
