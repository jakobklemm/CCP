//! # Tag

use anyhow::Result;
use lipsum::lipsum_with_rng;
use polodb_core::bson::doc;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::{store::Entity, DATABASE};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    name: String,
}

impl Tag {
    pub fn new(text: &str) -> Result<Self> {
        if let Ok(found) = DATABASE.get_one::<Self>(doc! {
            "name": text
        }) {
            return Ok(found);
        } else {
            let tag = Tag {
                name: text.to_string(),
            };
            let _ = DATABASE.insert(tag.clone())?;
            Ok(tag)
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        let tag = lipsum_with_rng(thread_rng(), 1);
        Self { name: tag }
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Entity for Tag {
    fn collection() -> &'static str {
        "tags"
    }

    fn to_document(&self, _schema: &tantivy::schema::Schema) -> anyhow::Result<tantivy::Document> {
        panic!("DO NOT PUT TAGS INTO FTS");
    }
}
