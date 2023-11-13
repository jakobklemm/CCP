// Main

use anyhow::Result;

mod application;
mod config;
mod entry;
mod handler;
mod interface;
mod processor;
mod terminal;
mod update;
mod util;

use crate::handler::Event;

use crate::entry::Id;
use crate::processor::{Job, Status, Timestamp};
use application::App;
use config::Config;
use crossterm::event::{self, KeyCode, KeyEventKind};
use entry::Entry;
use lazy_static::lazy_static;
use polodb_core::{bson::doc, Database};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};
use std::io::stderr;
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

    let col = DATABASE.collection::<Entry>("entries");
    let elms: Vec<Entry> = (1..100)
        .into_iter()
        .map(|x| Entry::new(format!("fkjakfs  {}", x)))
        .collect();

    let _ = col.insert_many(elms);

    let mut app = App::default();

    let term = Terminal::new(CrosstermBackend::new(stderr()))?;
    let events = handler::EventHandler::new(100);
    let mut tui = terminal::Terminal::new(term, events);
    tui.enter()?;

    while !app.should_quit() {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => {
                app.tick();
            }
            Event::Key(event) => update::update(&mut app, event),
            Event::Mouse(_m) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;

    // let job = Job::new("00:00:20", "00:01:35", "test.mp4", Id(45)).unwrap();
    // let s = job.execute();

    // while true {
    //     match s.recv().unwrap() {
    //         Status::Media(p) => println!("FFMPGE: {}", p),
    //         Status::Text(p) => println!("TXT: {}", p),
    //         Status::Completed(r) => {
    //             println!("transcript: {:?}", r);
    //             break;
    //         }
    //     }
    // }

    Ok(())
}
