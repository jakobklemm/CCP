//! # IDs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SID(Uuid);

impl Default for SID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EID(i32);

impl Default for EID {
    fn default() -> Self {
        Self(
            crate::config::Config::getNextID()
        )
    }
}
