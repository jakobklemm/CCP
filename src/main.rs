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
    // util::ensure_configured()?;

    // let mut app = App::default();

    // let term = Terminal::new(CrosstermBackend::new(stderr()))?;
    // let events = handler::EventHandler::new(250);
    // let mut tui = terminal::Terminal::new(term, events);
    // tui.enter()?;

    // while false {//!app.should_quit() {
    //     tui.draw(&mut app)?;
    //     match tui.events.next()? {
    //         Event::Tick => {}
    //         Event::Key(event) => update::update(&mut app, event),
    //         Event::Mouse(_m) => {}
    //         Event::Resize(_, _) => {}
    //     }
    // }

    // tui.exit()?;

    let instance = vlc::Instance::new().unwrap();
    let media = vlc::Media::new_path(&instance, "test.mp4").unwrap();
    let mdp = vlc::MediaPlayer::new(&instance).unwrap();

    let (tx, rx) = std::sync::mpsc::channel::<()>();
    
    let em = media.event_manager();
    let _ = em.attach(vlc::EventType::MediaStateChanged, move |e, _| {
        match e {
            vlc::Event::MediaStateChanged(s) => {
                println!("State : {:?}", s);
                if s == vlc::State::Ended || s == vlc::State::Error {
                    tx.send(()).unwrap();
                }
            },
            vlc::Event::MediaPlayerPaused => {
                println!("PAUSED!");
            }
            _ => (),
        }
    });

    mdp.set_media(&media);
    mdp.play().unwrap();

    println!("can PAUSE:    / {:?}", mdp.can_pause());

    std::thread::sleep(std::time::Duration::from_millis(5000));

    mdp.pause();

    std::thread::sleep(std::time::Duration::from_millis(5000));

    mdp.navigate(0);

    mdp.pause();

    std::thread::sleep(std::time::Duration::from_millis(5000));

    rx.recv().unwrap();

    Ok(())
}

fn on_pause(frame: i64) {
    println!("paused !!! {:?}", frame);
}
