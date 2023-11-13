//! # Search

use crate::entry::{Entry, Id};
use crate::interface::TextArea;
use ratatui::widgets::{List, ListItem, ListState};

#[derive(Debug, Clone)]
pub struct Search {
    input: TextArea,
    list: EntryList,
}

#[derive(Clone, Debug)]
pub struct EntryList {
    state: ListState,
    items: Vec<Entry>,
}
