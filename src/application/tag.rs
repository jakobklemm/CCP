//! # Tag

use serde::{Deserialize, Serialize};

use crate::store::Entity;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    name: String,
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

    fn to_document(&self, schema: &tantivy::schema::Schema) -> anyhow::Result<tantivy::Document> {
        panic!("DO NOT PUT TAGS INTO FTS");
    }
}
