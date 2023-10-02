//! # Entry

use crate::id::EID;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    id: EID,
    path: String,
    timestamp: usize,
}

impl Entry {
    fn new() -> Self {
        Self {
            id: EID::default(),
            path: String::from(""),
            timestamp: 42
        }
    }
}
