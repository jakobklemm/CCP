//! # Item

use std::fmt::Debug;

use anyhow::Result;
use serde::Serialize;
use tantivy::Document;

pub trait Entity: Clone + Debug + Serialize + Send + Sync + Unpin {
    fn collection() -> &'static str;
    fn to_document(&self) -> Result<Document>;
}
