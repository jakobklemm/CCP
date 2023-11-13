//! # Execute

use crate::{interface::Render, processor::Job};
use ratatui::widgets::{List, ListItem, ListState};

#[derive(Clone, Debug)]
pub struct Execute {
    list: JobList,
}

impl Render for Execute {
    fn render(&mut self, f: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {}

    fn input(&mut self, key: crossterm::event::KeyEvent) {}
}

#[derive(Clone, Debug, Default)]
struct JobList {
    pub state: ListState,
    pub items: Vec<Job>,
}

impl JobList {
    fn fetch(&mut self) {
        let col = crate::DATABASE.collection::<Job>("jobs");
        if let Ok(jobs) = col.find(None) {
            let fil = jobs.filter_map(|x| x.ok()).collect();
            self.items = fil;
        }
    }
}
