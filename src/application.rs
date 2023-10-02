//! # Application

#[derive(Debug)]
pub struct App {
    // Terminal System Specific States
    system: System,
    // TODO: Change temp application state
    counter: i64,
}

#[derive(Debug, Default)]
struct System {
    shutdown: bool,
    tick: i64,
}

impl App {
    pub fn terminate(&self) -> bool {
        self.system.shutdown
    }

    pub fn quit(&mut self) {
        self.system.shutdown = true;
    }

    pub fn counter(&self) -> i64 {
        self.counter
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn decrement(&mut self) {
        self.counter -= 1;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            counter: 0,
            system: System::default()
        }
    }
}
