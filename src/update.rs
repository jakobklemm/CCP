//! Update

use crate::application::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, key: KeyEvent) {
    match key.code {
        // KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // KeyCode::Char('j') if control(&key) => app.increment(),
        // KeyCode::Char('k') if control(&key) => app.decrement(),
        KeyCode::Char('l') if control(&key) => app.next(),
        KeyCode::Char('h') if control(&key) => app.previous(),
        KeyCode::Tab => {
            // app.home.active = (app.home.active + 1) % 2;
        }
        _ => {
            app.input(key);
        }
    }
}

pub fn control(key: &KeyEvent) -> bool {
    key.modifiers == KeyModifiers::CONTROL
}
