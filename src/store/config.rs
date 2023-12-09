//! # Config

use serde::{Deserialize, Serialize};
use tantivy::schema::Schema;

use super::Entity;

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

impl Entity for Config {
    fn collection() -> &'static str {
        "system"
    }

    fn to_document(&self, _schema: &Schema) -> anyhow::Result<tantivy::Document> {
        panic!("DO NOT INSERT CONFIG INTO FTS STORE.")
    }
}
