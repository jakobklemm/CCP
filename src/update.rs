//! Update

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::application::App;

pub fn update(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // KeyCode::Char('j') => app.increment(),
        // KeyCode::Char('k') => app.decrement(),
        KeyCode::Char('l') if key.modifiers == KeyModifiers::CONTROL => app.index += 1,
        KeyCode::Char('h') if key.modifiers == KeyModifiers::CONTROL => app.index -= 1,
        KeyCode::Tab => {
            app.home.active = (app.home.active + 1) % 2;
        }
        _ => {
            app.home.handle(key);
        }

    }
}
