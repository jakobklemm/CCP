//! # Indexed
//!
//! In Tantivy store.

use crate::DATABASE;
use anyhow::Error;
use serde::{Deserialize, Serialize};
use tantivy::Document;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Indexed {
    pub id: Vec<i64>,
}

impl TryFrom<Document> for Indexed {
    type Error = Error;

    fn try_from(value: Document) -> Result<Self, Self::Error> {
        let js = DATABASE.get_schema().to_json(&value);
        let prsd = serde_json::from_str::<Self>(&js)?;

        Ok(prsd)
    }
}
