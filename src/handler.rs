//! # Handler

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::time::{Duration, Instant};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::{self, JoinHandle as TokioHandle},
};

use std::sync::mpsc::{Sender, Receiver};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: Sender<Event>,
    /// Event receiver channel.
    receiver: Receiver<Event>,
    /// Event handler thread.
    handler: JoinHandle<()>,
}

impl EventHandler {
    pub fn new(rate: u64) -> Self {
        let delay = Duration::from_millis(rate);
        let (send, recv) = std::sync::mpsc::channel();
        let handler = {
            let sender = send.clone();
            thread::spawn(move || {
                let mut last = Instant::now();
                let timeout = delay.checked_sub(last.elapsed()).unwrap_or(delay);

                if event::poll(timeout).expect("no events available") {
                    match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                        CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                        CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                        _ => unimplemented!(),
                    }
                    .expect("failed to send terminal event")
                }

                if last.elapsed() >= delay {
                    sender.send(Event::Tick).expect("failed to send tick event");
                    last = Instant::now();
                }
            })
        };

        Self {
            sender: send,
            receiver: recv,
            handler,
        }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
