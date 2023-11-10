//! Application

#[derive(Debug, Default)]
pub struct App {
    counter: i64,
    quit: bool,
}

impl App {
    // Tick event of terminal
    pub fn tick(&self) {}

    // Getter 
    pub fn get_counter(&self) -> i64 {
        self.counter
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    // Quit event
    pub fn quit(&mut self) {
        self.quit = true;
    }

    // increment
    pub fn increment(&mut self) {
        self.counter += 1;
    }

    // decrement
    pub fn decrement(&mut self) {
        self.counter -= 1;
    }
}
