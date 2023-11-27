//! Update

use crate::application::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => {
            app.quit();
        }
        KeyCode::Char('l') if control(&key) => app.next(),
        KeyCode::Char('h') if control(&key) => app.previous(),
        _ => {
            app.input(key);
        }
    }
}

pub fn control(key: &KeyEvent) -> bool {
    key.modifiers == KeyModifiers::CONTROL
}
