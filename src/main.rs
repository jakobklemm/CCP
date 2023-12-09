// Main

use anyhow::Result;
use store::Database;

mod application;
mod handler;
mod interface;
mod store;
mod terminal;
mod update;
mod util;

use crate::handler::Event;

use application::App;
// use crossterm::event::{self, KeyCode, KeyEventKind};
use lazy_static::lazy_static;
use polodb_core::bson::doc;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::stderr;

lazy_static! {
    pub static ref ROOT: String = std::env::var("CCP_ROOT").unwrap_or("/temp".to_string());
    pub static ref DATABASE: Database = Database::new().unwrap();
}

fn main() -> Result<()> {
    util::ensure_configured()?;

    let mut app = App::default();
    // let _ = manual();
    // println!("test");

    // for _ in 1..15 {
    //     let e = Entry::default();
    //     let _ = DATABASE.insert_indexed(e);
    //     let t = Tag::default();
    //     let _ = DATABASE.insert(t);
    // }

    // let res: Vec<Entry> = DATABASE.search("*")?;

    // println!("{:?}", res);

    // let res: Vec<Entry> = DATABASE.search("id:5")?;
    // let five = res.get(0).unwrap();
    // let e = actions::open_vlc(five.clone());
    // println!("{:?}", e);

    // let j = Job::default();
    // println!("JOB: {:?}", j);
    // let r = crate::application::processor::execute(j);
    //
    // for msg in r.iter() {
    //     println!("PROG: {:?}", msg);
    // }

    // let job = Job::default();
    //
    // let mut count = 0;
    //
    // for j in DATABASE.get_all::<Job>().unwrap() {
    //     count += 1;
    //     println!("{:?}", j.unwrap());
    // }
    //
    // println!("PART 1: {}", count);
    //
    // let _ = DATABASE.update_many::<Job>(
    //     doc! {
    //         "uid": "aaeab785-249b-4565-8652-b1e274b9a312",
    //     },
    //     doc! {
    //         "$set": doc! {
    //         "done": true
    //         }
    //     },
    // );
    //
    // let mut count = 0;
    //
    // for j in DATABASE
    //     .get_many::<Job>(doc! {
    //         "done": false
    //     })
    //     .unwrap()
    // {
    //     count += 1;
    //     println!("{:?}", j.unwrap());
    // }
    //
    // println!("PART 2: {}", count);

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

    Ok(())
}
