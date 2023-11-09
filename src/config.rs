//! Config

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "_id")]
    id: String,
    pub counter: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            id: String::from("CONFIG"),
            counter: 1,
        }
    }
}
