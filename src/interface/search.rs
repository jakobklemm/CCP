//! # Search

use crate::interface::{Render, TextArea};
use crate::update::control;
use crate::DATABASE;
use anyhow::Result;
use chrono::SecondsFormat;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{
    layout::Rect,
    prelude::Frame,
    widgets::{List, ListItem},
};

use super::list::ItemList;
use crate::application::{actions, Entry};

#[derive(Debug, Clone)]
pub struct Search {
    input: TextArea,
    text: String,
    list: ItemList<Entry>,
}

impl Default for Search {
    fn default() -> Self {
        let mut input = TextArea::default();

        input.set_cursor_style(Style::default());
        input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        let mut items = Vec::new();
        if let Ok(dflt) = DATABASE.search("*") {
            items = dflt;
        }

        Self {
            input,
            text: String::new(),
            list: ItemList::new(items),
        }
    }
}

impl Render for Search {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        self.render_bar(f, layout[0]);
        self.render_list(f, layout[1]);
    }

    fn input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('j') if control(&key) => {
                self.list.next();
            }
            KeyCode::Char('k') if control(&key) => {
                self.list.previous();
            }
            KeyCode::Up => {
                self.list.previous();
            }
            KeyCode::Down => {
                self.list.next();
            }
            KeyCode::Char('o') if control(&key) => {
                if let Some(e) = self.list.get() {
                    self.text = e.out_path().unwrap();
                    let _ = actions::open_nautilus(e);
                }
            }
            KeyCode::Enter => {
                // TODO Open event
                if let Some(e) = self.list.get() {
                    self.text = e.out_path().unwrap();
                    let _ = actions::open_vlc(e);
                }
            }
            _ => {
                self.input.input(key);
                // not nice workaroud
                for l in self.input.lines() {
                    self.text = l.to_string();
                    break;
                }
                if self.text.len() >= 1 {
                    match DATABASE.search(&self.text) {
                        Ok(elems) => {
                            self.list.set(elems);
                            self.list.select(None);
                        }
                        Err(_) => {
                            // TODO: Maybe reset list to empty
                            self.list.set(Vec::new());
                            self.list.select(None);
                        }
                    }
                }
            }
        }
    }
}

impl Search {
    fn render_bar(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(5), Constraint::Min(1)])
            .direction(Direction::Horizontal)
            .split(area);

        let para = Paragraph::new(format!(" Search: "))
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

        f.render_widget(para, layout[0]);
        f.render_widget(self.input.widget(), layout[1]);
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        let listed: Vec<ListItem> = self
            .list
            .items()
            .iter()
            .map(|x| ListItem::new(x.search_str()).style(Style::default()))
            .collect();

        let list = List::new(listed)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::default().fg(Color::Gray));

        f.render_stateful_widget(list, area, &mut self.list.state);
    }
}
