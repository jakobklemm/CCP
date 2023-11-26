// Main

use anyhow::Result;
use database::Database as DB;
use metadata::Metadata;
use tantivy::schema::Schema;
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy, Searcher};

mod application;
mod config;
mod database;
mod entry;
mod handler;
mod interface;
mod metadata;
mod processor;
mod root;
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
use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static! {
    pub static ref DATABASE: Database = {
        let root = std::env::var("CCP_ROOT").unwrap_or("/database".to_string());
        Database::open_file(format!("{}/ccp-polo.db", root)).expect("Unable to open database")
    };
    pub static ref ROOT: String = std::env::var("CCP_ROOT").unwrap_or("/database".to_string());
    pub static ref SCHEMA: Schema = DB::schema();
    pub static ref INDEX: Index = {
        let path = format!("{}/db/", ROOT.as_str());
        let _ = std::fs::create_dir_all(path.clone());

        let index = match Index::create_in_dir(path.clone(), SCHEMA.to_owned()) {
            Ok(i) => i,
            Err(_) => Index::open_in_dir(path).expect("Unable to open Tantivy Database"),
        };

        index
    };
    pub static ref WRITER: Arc<Mutex<IndexWriter>> = {
        let w = INDEX
            .writer(25_000_000)
            .expect("Unable to create Tantivy writer");
        Arc::new(Mutex::new(w))
    };
    pub static ref SEARCHER: IndexReader = {
        INDEX
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .expect("Unable to create Tantivy reader")
    };
}

fn main() -> Result<()> {
    util::ensure_configured()?;

    // let meta = Metadata::new("ingest/test.mp4".to_string());
    // println!("{:?}", meta);

    // let col = DATABASE.collection::<Entry>("entries");
    // let elms: Vec<Entry> = (1..100)
    //     .into_iter()
    //     .map(|x| Entry::new(format!("fkjakfs  {}", x)))
    //     .collect();
    //
    // let _ = col.insert_many(elms);
    //

    // let mut db = DB::new()?;
    let mut app = App::default();
    // let _ = DB::random();
    let s = DB::search("materia")?;
    for e in &s {
        println!("{:?}", e);
    }

    println!("Count: {}", s.len());
    //
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

    // let job = Job::new("00:00:20", "00:01:35", "test.mp4", "test", Vec::new()).unwrap();
    // let s = job.execute();
    //
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
    //
    Ok(())
}
