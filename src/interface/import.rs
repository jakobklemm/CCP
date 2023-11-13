//! # Import

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    interface::{Render, TextArea},
    update::control,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Style},
    widgets::{block::Title, Block, BorderType, Borders, Paragraph},
    Frame,
};

const INPUTS: usize = 5;

/// TODO: Add Tags list and file tree.
#[derive(Debug, Clone)]
pub struct Import {
    current: usize,
    start: TextArea,
    end: TextArea,
    title: TextArea,
    selector: TextArea,
    language: TextArea,
}

impl Render for Import {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let accept = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Green));

        match self.current {
            0 => {
                self.reset_all();
                self.start.set_block(accept.title(" Start Time "));
            }
            1 => {
                self.reset_all();
                self.end.set_block(accept.title(" End Time "));
            }
            2 => {
                self.reset_all();
                self.language.set_block(accept.title(" Language "));
            }
            3 => {
                self.reset_all();
                self.title.set_block(accept.title(" Enter Title "));
            }
            4 => {
                self.reset_all();
                self.selector.set_block(accept.title(" Enter Tags "));
            }
            _ => {}
        }

        let layout = Layout::default()
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .direction(Direction::Horizontal)
            .split(area);

        self.render_tree(f, layout[0]);
        self.render_input(f, layout[1]);
        self.render_meta(f, layout[2]);
    }

    /// TODO: Handle inner swap to lower block
    fn input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                self.focus_next();
            }
            KeyCode::Down if control(&key) => {
                self.focus_next();
            }
            KeyCode::Up if control(&key) => {
                self.focus_prev();
            }
            KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => {
                // TODO: Submit job
            }
            _ => {
                match self.current {
                    0 => self.start.input(key),
                    1 => self.end.input(key),
                    2 => self.language.input(key),
                    3 => self.title.input(key),
                    4 => self.selector.input(key),
                    _ => false,
                };
            }
        }
    }
}

impl Import {
    fn reset_all(&mut self) {
        let block = Block::default()
            .style(Style::default().fg(Color::Gray))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        self.start.set_block(block.clone().title(" Start Time "));
        self.end.set_block(block.clone().title(" End Time "));
        self.title.set_block(block.clone().title(" Enter Title "));
        self.language.set_block(block.clone().title(" Language "));
        self.selector.set_block(block.clone().title(" Enter Tags "));
    }

    fn focus_next(&mut self) {
        self.current = (self.current + 1) % INPUTS;
    }

    fn focus_prev(&mut self) {
        self.current = (INPUTS + self.current - 1) % INPUTS;
    }

    fn render_tree(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(
            Paragraph::new("TODO: Tree Viewer").block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            area,
        );
    }

    fn render_input(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        self.render_upper(f, layout[0]);
        self.render_lower(f, layout[1]);
    }

    fn render_upper(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .direction(Direction::Horizontal)
            .split(area);

        self.render_simples(f, layout[0]);
        self.render_text(f, layout[1]);
    }

    fn render_simples(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let block = Block::default()
            .style(Style::default().fg(Color::Reset))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let identity = Paragraph::new(format!(" {}", 42))
            .block(block.clone().title(" Entity ID "))
            .style(Style::default().fg(Color::DarkGray));

        let name = Paragraph::new(format!(" test.mp4"))
            .block(block.clone().title(" File Name "))
            .style(Style::default().fg(Color::DarkGray));

        let ts = Paragraph::new(format!(
            " {:?}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ))
        .block(block.clone().title(" Timestamp "))
        .style(Style::default().fg(Color::DarkGray));

        f.render_widget(identity, layout[0]);
        f.render_widget(name, layout[1]);
        f.render_widget(ts, layout[2]);
        f.render_widget(self.start.widget(), layout[3]);
        f.render_widget(self.end.widget(), layout[4]);
        f.render_widget(self.language.widget(), layout[5]);
    }
    fn render_text(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(self.title.widget(), area);
    }

    fn render_lower(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        self.render_bar(f, layout[0]);
        self.render_list(f, layout[1]);
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(
            Paragraph::new("TODO: Tags list").block(Block::default().borders(Borders::ALL)),
            area,
        );
    }

    fn render_bar(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(7), Constraint::Min(0)])
            .direction(Direction::Horizontal)
            .split(area);

        let para = Paragraph::new(" Tags: ").style(Style::default()).block(
            Block::default()
                .style(Style::default())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        f.render_widget(para, layout[0]);
        f.render_widget(self.selector.widget(), layout[1]);
    }

    fn render_meta(&mut self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Magenta))
            .title(Title::from(" Metadata ").alignment(Alignment::Center));

        let para = Paragraph::new(" GET ffprobe ")
            .block(block)
            .style(Style::default());

        f.render_widget(para, area);
    }
}

impl Default for Import {
    fn default() -> Self {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let mut start = TextArea::default();
        start.set_placeholder_text(" Start HH:MM:SS");
        start.set_block(block.clone().title(" Start Time "));
        let mut end = TextArea::default();
        end.set_placeholder_text(" End HH:MM:SS");
        end.set_block(block.clone().title(" End Time "));
        let mut title = TextArea::default();
        title.set_placeholder_text(" Enter Title");
        title.set_block(block.clone().title(" Enter Title "));
        let mut language = TextArea::default();
        language.set_placeholder_text(" de, en");
        language.set_block(block.clone().title(" Language "));
        let mut selector = TextArea::default();
        selector.set_placeholder_text(" Enter tags");
        selector.set_block(block.clone());

        Self {
            current: 0,
            start,
            end,
            title,
            selector,
            language,
        }
    }
}
