//! # Config

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    globalCounter: usize
}


impl Config {
    fn getNextID() -> usize {
        let collection = crate::DATABASE.collection::<Config>("config");
        0
    } 
}

impl Default for Config {
    fn default() -> Self {
        Self {
            globalCounter: 0,
        }
    }
}
