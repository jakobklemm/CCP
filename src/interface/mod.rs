//! # Interface
//!

use crate::application::{App, EntryList};
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Frame,
};
use tui_textarea::TextArea as TuiTextArea;

mod dashboard;
use dashboard::Dashboard;

mod search;
use search::Search;

mod import;
use import::Import;

mod execute;
use execute::Execute;

mod settings;
use settings::Settings;

mod header;
use header::Header;

mod footer;
use footer::Footer;

pub type TextArea = TuiTextArea<'static>;

pub const STATES: i32 = 5;

#[derive(Debug, Clone)]
pub struct Interface {
    header: Header,
    footer: Footer,
    state: State,
}

impl Interface {
    pub fn next(&mut self) {
        self.header.next();
    }

    pub fn previous(&mut self) {
        self.header.previous();
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Dashboard(Dashboard),
    Search(Search),
    Import(Import),
    Execute(Execute),
    Settings(Settings),
}

impl Render for Interface {
    fn render(&self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .direction(Direction::Vertical)
            .split(f.size());

        self.header.render(f, layout[0]);
    }
    fn input(&mut self, key: KeyEvent) {}
}

/// Workaroud trait instead of Widget
pub trait Render {
    fn render(&self, f: &mut Frame, area: Rect);
    fn input(&mut self, key: KeyEvent);
}

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .constraints([Constraint::Min(0)])
        .split(f.size());

    // TODO: Render interface from App
}
