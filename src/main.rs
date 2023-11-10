// Main

use anyhow::Result;

mod application;
mod config;
mod entry;
mod interface;
mod processor;
mod handler;
mod terminal;
mod update;
mod util;

use crate::handler::Event;

use config::Config;
use entry::Entry;
use lazy_static::lazy_static;
use polodb_core::{bson::doc, Database};
use application::App;
use std::io::stderr;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

use std::time::Duration;

lazy_static! {
    pub static ref DATABASE: Database = {
        let root = std::env::var("CCP_ROOT").unwrap_or("/database".to_string());
        Database::open_file(format!("{}/ccp-polo.db", root)).expect("Unable to open database")
    };
    pub static ref ROOT: String = std::env::var("CCP_ROOT").unwrap_or("/database".to_string());
}

fn main() -> Result<()> {
    util::ensure_configured()?;

    let mut app = App::default();

    let term = Terminal::new(CrosstermBackend::new(stderr()))?;
    let events = handler::EventHandler::new(250);
    let mut tui = terminal::Terminal::new(term, events);
    tui.enter()?;

    while !app.should_quit() {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(event) => update::update(&mut app, event),
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;

    Ok(())
}
