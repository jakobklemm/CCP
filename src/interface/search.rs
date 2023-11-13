//! # Search

use crate::entry::{Entry, Id};
use crate::interface::{Render, TextArea};
use crate::update::control;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{
    layout::Rect,
    prelude::Frame,
    widgets::{List, ListItem, ListState},
};

#[derive(Debug, Clone)]
pub struct Search {
    input: TextArea,
    text: String,
    list: EntryList,
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

        Self {
            input,
            text: String::new(),
            list: Default::default(),
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
            KeyCode::Enter => {
                // TODO Open event
            }
            _ => {
                self.input.input(key);
                // not nice workaroud
                for l in self.input.lines() {
                    self.text = l.to_string();
                    break;
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
            .items
            .iter()
            .map(|x| {
                let t = format!("{} - {} - {}", x.id.0, x.title, x.text);
                ListItem::new(t).style(Style::default().fg(Color::Green))
            })
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

#[derive(Clone, Debug)]
pub struct EntryList {
    pub state: ListState,
    pub items: Vec<Entry>,
}

impl EntryList {
    fn fetch(&mut self) -> Result<usize> {
        let col = crate::DATABASE.collection::<Entry>("entries");
        let entries: Vec<Entry> = col.find(None)?.filter_map(|x| x.ok()).collect();
        self.items = entries;
        Ok(self.items.len())
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (self.items.len() + i - 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl Default for EntryList {
    fn default() -> Self {
        let mut list = Self {
            state: Default::default(),
            items: Default::default(),
        };
        // TODO: Error handling
        let _ = list.fetch();
        list
    }
}
