//! # Header

use std::time::Instant;

use crate::interface::{Render, State, STATES};
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{block::Title, Block, BorderType, Borders, Paragraph, Tabs},
    Frame,
};

#[derive(Debug, Clone, Default)]
pub struct Header {
    selected: i32,
    fps: f64,
    frames: u32,
    start: Option<Instant>,
}

impl Render for Header {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let bar = Layout::default()
            .constraints([Constraint::Percentage(94), Constraint::Percentage(4)])
            .direction(Direction::Horizontal)
            .split(area);

        self.render_tabs(f, bar[0]);
        self.render_fps(f, bar[1]);
    }
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

    pub fn tick(&mut self) {
        self.frames += 1;
        let now = Instant::now();
        match self.start {
            Some(inst) => {
                let elapsed = (now - inst).as_secs_f64();
                if elapsed > 1.0 {
                    self.fps = self.frames as f64 / elapsed;
                    self.start = Some(now);
                    self.frames = 0;
                }
            }
            None => {}
        }
    }

    fn render_fps(&self, f: &mut Frame, area: Rect) {
        let percision = {
            let width = f.size().width;
            if width > 250 {
                3
            } else if width > 200 {
                2
            } else if width > 150 {
                1
            } else {
                0
            }
        };

        let para = Paragraph::new(format!("{:.*}", percision, self.fps))
            .block(
                Block::default()
                    .title(" FPS ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::new().fg(Color::Gray))
            .alignment(Alignment::Center);

        f.render_widget(para, area);
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let sections = State::get_states()
            .iter()
            .cloned()
            .map(Line::from)
            .collect();

        let outer = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::LightRed))
            .title(Title::from(" CENTRAL CONTENT PROCESSOR ").alignment(Alignment::Center))
            .title(Title::from(" TAB SECRET ").alignment(Alignment::Left))
            .title(Title::from(" V0.1.0 ").alignment(Alignment::Right));

        let tabs = Tabs::new(sections)
            .block(outer)
            .style(Style::default().fg(Color::Red))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .divider(" | ")
            .select(self.selected.try_into().unwrap_or(0));

        f.render_widget(tabs, area);
    }
}
