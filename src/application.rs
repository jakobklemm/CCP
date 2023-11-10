//! Application

use tui_textarea::{TextArea};
use std::time::Instant;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{style::Style, widgets::{Borders, Block}};

#[derive(Debug, Default)]
pub struct App<'a> {
    counter: i64,
    pub index: i64,
    pub home: HomeScreen<'a>,
    pub fps: f64,
    pub frames: u32,
    pub start: Option<Instant>,
    quit: bool,
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

    // Getter 
    pub fn get_counter(&self) -> i64 {
        self.counter
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn get_index(&self) -> usize {
        self.index as usize
    }

    // Quit event
    pub fn quit(&mut self) {
        self.quit = true;
    }

    // increment
    pub fn increment(&mut self) {
        self.counter += 1;
    }

    // decrement
    pub fn decrement(&mut self) {
        self.counter -= 1;
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
            _ => todo!()
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
                        .title(format!("SECOND Input Box"))
                );
                area
            }
        }
    }
}
