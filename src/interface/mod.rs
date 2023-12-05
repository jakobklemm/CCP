//! # Interface
//!

use crate::application::App;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    style::{Color, Style},
    widgets::Paragraph,
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

mod list;

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
        self.state = match self.state {
            State::Dashboard(_) => State::Search(Search::default()),
            State::Search(_) => State::Import(Import::default()),
            State::Import(_) => State::Execute(Execute::default()),
            State::Execute(_) => State::Settings(Settings::default()),
            State::Settings(_) => State::Dashboard(Dashboard::default()),
        };
    }

    pub fn previous(&mut self) {
        self.header.previous();
        self.state = match self.state {
            State::Dashboard(_) => State::Settings(Settings::default()),
            State::Search(_) => State::Dashboard(Dashboard::default()),
            State::Import(_) => State::Search(Search::default()),
            State::Execute(_) => State::Import(Import::default()),
            State::Settings(_) => State::Execute(Execute::default()),
        };
    }

    pub fn tick(&mut self) {
        self.header.tick();
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

impl State {
    pub fn get_states() -> [&'static str; STATES as usize] {
        ["Dashboard", "Search", "Import", "Execute", "Settings"]
    }
}

impl Render for Interface {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        if area.width <= 100 {
            f.render_widget(
                Paragraph::new("Unable to display, please increase size.")
                    .style(Style::default().fg(Color::Red)),
                area,
            );
            return;
        }
        let layout = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .direction(Direction::Vertical)
            .split(area);

        self.header.render(f, layout[0]);
        self.state.render(f, layout[1]);
        self.footer.render(f, layout[2]);
    }

    fn input(&mut self, key: KeyEvent) {
        self.state.input(key);
    }
}

impl Render for State {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        match self {
            State::Dashboard(d) => d.render(f, area),
            State::Search(s) => s.render(f, area),
            State::Import(i) => i.render(f, area),
            State::Execute(e) => e.render(f, area),
            State::Settings(s) => s.render(f, area),
        }
    }

    fn input(&mut self, key: KeyEvent) {
        match self {
            State::Dashboard(d) => d.input(key),
            State::Search(s) => s.input(key),
            State::Import(i) => i.input(key),
            State::Execute(e) => e.input(key),
            State::Settings(s) => s.input(key),
        }
    }
}

impl Default for Interface {
    fn default() -> Self {
        Self {
            header: Default::default(),
            footer: Default::default(),
            state: Default::default(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Dashboard(Dashboard::default())
    }
}

/// Workaroud trait instead of Widget
pub trait Render {
    fn render(&mut self, f: &mut Frame, area: Rect);
    fn input(&mut self, key: KeyEvent);
}

pub fn render(app: &mut App, f: &mut Frame) {
    app.ui.render(f, f.size());
}
