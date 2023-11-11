//! Update

use crate::application::{App, Interface};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, key: KeyEvent) {
    match key.code {
        // KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('j') if control(&key) => app.increment(),
        KeyCode::Char('k') if control(&key) => app.decrement(),
        KeyCode::Char('l') if control(&key) => app.next(),
        KeyCode::Char('h') if control(&key) => app.previous(),
        KeyCode::Tab => {
            app.home.active = (app.home.active + 1) % 2;
        }
        _ => {
            //app.home.handle(key);
            handle_other(app, key);
        }
    }
}

fn handle_other(app: &mut App, key: KeyEvent) {
    match &mut app.interface {
        Interface::Search{ref mut input, list} => {
            (*input).input(key);
            // TODO: rebuild list
        }
        _ => {}
    }
}

fn control(key: &KeyEvent) -> bool {
    key.modifiers == KeyModifiers::CONTROL
}
