//! # Entry

use crate::id::EID;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    id: EID,
    path: String,
    timestamp: usize,
}
