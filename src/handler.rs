//! Event Handler

use crossterm::event::{self, Event as CrossEvent, KeyEvent, MouseEvent, KeyEventKind};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use anyhow::Result;

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16)
}

#[derive(Debug)]
pub struct EventHandler {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    handle: JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tps: u64) -> Self {
        let rate = Duration::from_millis(tps);
        let (tx, rx) = channel();
        let handle = {
            let sender = tx.clone();
            thread::spawn(move || {
                let mut last = Instant::now();
                loop {
                    let timeout = rate.checked_sub(last.elapsed()).unwrap_or(rate);

                    if event::poll(timeout).expect("no events found") {
                        match event::read().expect("unable to read event") {
                            CrossEvent::Key(e) => {
                                if e.kind == KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    // can only be relased
                                    Ok(())
                                }
                            }
                            CrossEvent::Mouse(e) => {
                                sender.send(Event::Mouse(e))
                            }
                            CrossEvent::Resize(w, h) => {
                                sender.send(Event::Resize(w, h))
                            }
                            _ => {
                                // focus lost & gained events
                                unimplemented!()
                            }
                        }.expect("outer error")
                    }

                    if last.elapsed() >= rate {
                        sender.send(Event::Tick).expect("Channel failed");
                        last = Instant::now();
                    }
                }
            })
        };

        Self {
            sender: tx,
            receiver: rx,
            handle,
        }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
