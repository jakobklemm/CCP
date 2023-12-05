//! # Item

use std::fmt::Debug;

use anyhow::Result;
use serde::Serialize;
use tantivy::{schema::Schema, Document};

pub trait Entity: Clone + Debug + Serialize {
    fn collection() -> &'static str;
    fn to_document(&self, schema: &Schema) -> Result<Document>;
}
