//! # List

use std::fmt::Debug;

use ratatui::widgets::ListState;

#[derive(Clone, Debug)]
pub struct ItemList<E: Debug + Clone> {
    pub(crate) state: ListState,
    items: Vec<E>,
}

impl<E: Clone + Debug> Default for ItemList<E> {
    fn default() -> Self {
        Self {
            state: ListState::default(),
            items: vec![],
        }
    }
}

impl<E: Debug + Clone> ItemList<E> {
    pub fn new(items: Vec<E>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn get(&self) -> Option<E> {
        Some(self.items.get(self.state.selected()?)?.clone())
    }

    pub fn items(&self) -> &Vec<E> {
        &self.items
    }

    pub fn select(&mut self, select: Option<usize>) {
        self.state.select(select)
    }

    pub fn set(&mut self, items: Vec<E>) {
        self.items = items;
    }

    pub fn next(&mut self) {
        if self.items.len() == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.len() == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (self.items.len() + i - 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }
}
