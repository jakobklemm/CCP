//! Application

use crate::entry::Entry;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::Style,
    widgets::{Block, Borders, List, ListState},
};
use std::time::Instant;
use tui_textarea::TextArea;

#[derive(Debug, Default)]
pub struct App<'a> {
    counter: i64,
    pub index: i64,
    pub home: HomeScreen<'a>,
    pub fps: f64,
    pub frames: u32,
    pub start: Option<Instant>,
    pub interface: Interface,
    quit: bool,
}

#[derive(Debug)]
pub enum Interface {
    Dashboard {
        count: u64,
        tags: u64,
        size: f64,
    },
    Search {
        input: TextArea<'static>,
        list: EntryList,
    },
    Import {},
    Tags {},
    People {},
    Import {},
    Export {},
}

impl Default for Interface {
    fn default() -> Self {
        Self::Dashboard {
            count: 0,
            tags: 0,
            size: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct EntryList {
    pub state: ListState,
    pub items: Vec<Entry>,
}

impl EntryList {
    pub fn new() -> Result<Self> {
        let col = crate::DATABASE.collection::<Entry>("entries");
        let entries: Vec<Entry> = col.find(None)?.filter_map(|x| x.ok()).collect();
        Ok(Self {
            state: ListState::default(),
            items: entries,
        })
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (self.items.len() + i - 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl<'a> App<'a> {
    // Tick event of terminal
    pub fn tick(&mut self) {
        if self.frames == 0 {
            self.start = Some(Instant::now());
        }
        self.frames += 1;
        let now = Instant::now();
        let elapsed = (now - self.start.unwrap_or(Instant::now())).as_secs_f64();
        if elapsed >= 1.0 {
            self.fps = self.frames as f64 / elapsed as f64;
            self.start = Some(now);
            self.frames = 0;
        }
    }

    pub fn next(&mut self) {
        match &mut self.interface {
            Interface::Dashboard { count, tags, size } => {
                self.interface = Interface::Search {
                    input: TextArea::default(),
                    list: EntryList::new().unwrap(),
                };
            }
            Interface::Search { input, list } => {}
            _ => self.interface = Interface::Tags {},
        }
    }

    pub fn previous(&mut self) {
        match &mut self.interface {
            Interface::Dashboard { count, tags, size } => {
                self.interface = Interface::Tags {};
            }
            Interface::Search { input, list } => {
                self.interface = Interface::Dashboard {
                    count: 0,
                    tags: 0,
                    size: 0.0,
                }
            }
            _ => {
                self.interface = Interface::Search {
                    input: TextArea::default(),
                    list: EntryList::new().unwrap(),
                }
            }
        }
    }

    // Getter
    pub fn get_counter(&self) -> i64 {
        self.counter
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn get_index(&self) -> usize {
        match self.interface {
            Interface::Dashboard { .. } => 0,
            Interface::Search { .. } => 1,
            _ => 2,
        }
    }

    // Quit event
    pub fn quit(&mut self) {
        self.quit = true;
    }

    // increment
    pub fn increment(&mut self) {
        match &mut self.interface {
            Interface::Search { input, list } => {
                list.next();
            }
            _ => {}
        }
    }

    // decrement
    pub fn decrement(&mut self) {
        match &mut self.interface {
            Interface::Search { input, list } => {
                list.previous();
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct HomeScreen<'a> {
    pub active: usize,
    pub first: TextArea<'a>,
    pub second: TextArea<'a>,
}

impl<'a> HomeScreen<'a> {
    pub fn handle(&mut self, key: KeyEvent) {
        match self.active % 2 {
            0 => self.first.input(key),
            1 => self.second.input(key),
            _ => todo!(),
        };
    }
}

impl<'a> Default for HomeScreen<'a> {
    fn default() -> Self {
        let mut area = TextArea::default();
        area.set_cursor_line_style(Style::default());
        area.set_placeholder_text("First Text Box");
        Self {
            active: 0,
            first: area,
            second: {
                let mut area = TextArea::default();
                area.set_cursor_line_style(Style::default());
                area.set_placeholder_text("First Text Box");
                area.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("SECOND Input Box")),
                );
                area
            },
        }
    }
}
