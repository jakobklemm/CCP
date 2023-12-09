//! Terminal

use crate::application::App;
use crate::handler::EventHandler;
use crate::interface;
use crate::util;
use anyhow::Result;


pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

pub struct Terminal {
    terminal: CrosstermTerminal,
    pub events: EventHandler,
}

impl Terminal {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn enter(&mut self) -> Result<()> {
        util::terminal_startup()?;
        // panic::set_hook(Box::new(move |_panic| {
        //     let _ = util::terminal_shutdown();
        // }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| interface::render(app, frame))?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        util::terminal_shutdown()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
