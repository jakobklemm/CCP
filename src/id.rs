//! # IDs

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SID (Uuid);

impl Default for SID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EID (usize);

impl Default for EID {
    fn default() -> Self {
        Self(0)
    }
}
