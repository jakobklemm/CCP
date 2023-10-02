use anyhow::Result;
use lazy_static::lazy_static;
use polodb_core::Database;
use std::env;

// Application state logic
pub mod application;

// "Renderer"
pub mod interface;

// Event handler
pub mod handler;
use handler::Event;

// Updater - Combine Events and App
pub mod update;
use update::update;

// Abstract TUI functions
pub mod terminal;

lazy_static! {
    static ref DATABASE: Database = Database::open_file(format!(
        "{}/dcp.db",
        env::var("DCP_ROOT").unwrap_or(String::from("/database"))
    ))
    .unwrap();
}

use ratatui::{backend::CrosstermBackend, Terminal as RatTerm};

fn main() -> Result<()> {
    let mut app = application::App::default();
    let backend = CrosstermBackend::new(std::io::stderr());
    let term = RatTerm::new(backend)?;
    let events = handler::EventHandler::new(250);
    let mut terminal = terminal::Terminal::new(term, events);

    initialize_panic_handler();
    terminal::startup()?;

    while !app.terminate() {
        terminal.draw(&mut app)?;
        match terminal.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    terminal::shutdown()?;
    Ok(())
}

pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        terminal::shutdown().unwrap();
        original_hook(panic_info);
    }));
}
