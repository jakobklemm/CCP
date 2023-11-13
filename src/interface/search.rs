//! # Search

use crate::entry::{Entry, Id};
use crate::interface::TextArea;
use ratatui::widgets::{List, ListItem, ListState};

#[derive(Debug, Clone)]
pub struct Search {
    input: TextArea,
    list: EntryList,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            input: Default::default(),
            list: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EntryList {
    state: ListState,
    items: Vec<Entry>,
}

impl Default for EntryList {
    fn default() -> Self {
        Self {
            state: Default::default(),
            items: Default::default(),
        }
    }
}
