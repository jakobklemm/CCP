//! # Config

use super::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(rename = "_id")]
    id: String,
    counter: i64,
}

impl Config {
    pub fn get_counter(&self) -> i64 {
        self.counter
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            id: "CONFIG".to_string(),
            counter: 1,
        }
    }
}

impl Item for Config {
    fn collection() -> &'static str {
        "system"
    }

    fn to_document(&self) -> anyhow::Result<tantivy::Document> {
        panic!("DO NOT INSERT CONFIG INTO FTS STORE.")
    }
}
