//! # Entry
//!
//! In PoloDB store

use std::fs::metadata;

use super::{id::Id, indexed::Indexed, job::Job, tag::Tag, timestamp::Timestamp};
use crate::{store::Entity, DATABASE};
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use lipsum::{lipsum_with_rng, lipsum_words_with_rng};
use polodb_core::bson::doc;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use tantivy::{schema::Schema, DateTime, Document};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "_id")]
    id: Id,
    title: String,
    description: String,
    transcript: String,
    date: NaiveDate,
    tags: Vec<Tag>,
    file: FileInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileInfo {
    // in bytes
    size: u64,
    // in seconds
    duration: i32,
}

impl Entry {
    pub fn new(
        id: Id,
        file: &str,
        title: String,
        description: String,
        transcript: String,
        date: NaiveDate,
        tags: Vec<Tag>,
        duration: Timestamp,
    ) -> Result<Self> {
        Ok(Self {
            id,
            title,
            transcript,
            description,
            date,
            tags,
            file: FileInfo::new(file, duration)?,
        })
    }

    pub fn search_str(&self) -> String {
        format!(
            "{} - {} - {} - {} - {:?}",
            self.id.get(),
            self.date.format("%d-%m-%Y").to_string(),
            self.title,
            self.description,
            self.tags
        )
    }

    pub fn out_path(&self) -> Result<String> {
        self.id.data_path()
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            size: Default::default(),
            duration: Default::default(),
        }
    }
}

impl FileInfo {
    fn new(file: &str, duration: Timestamp) -> Result<Self> {
        let meta = metadata(file)?;
        Ok(Self {
            size: meta.len(),
            duration: duration.to_seconds(),
        })
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: Id::default(),
            title: lipsum_words_with_rng(thread_rng(), 6),
            description: lipsum_words_with_rng(thread_rng(), 18),
            transcript: lipsum_with_rng(thread_rng(), 64),
            date: Default::default(),
            tags: Default::default(),
            file: Default::default(),
        }
    }
}

impl TryFrom<Document> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: Document) -> std::prelude::v1::Result<Self, Self::Error> {
        let indx: Indexed = value.try_into()?;
        Ok(indx.try_into()?)
    }
}

impl TryFrom<Indexed> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: Indexed) -> std::prelude::v1::Result<Self, Self::Error> {
        for i in value.id {
            let doc = DATABASE.get_one::<Entry>(doc! {"_id": i})?;
            return Ok(doc);
        }

        Err(anyhow!("Database invalid!"))
    }
}

impl Entity for Entry {
    fn collection() -> &'static str {
        "entries"
    }

    fn to_document(&self, schema: &Schema) -> Result<Document> {
        let id = schema.get_field("id")?;
        let title = schema.get_field("title")?;
        let desc = schema.get_field("description")?;
        let transcript = schema.get_field("transcript")?;
        let tags = schema.get_field("tags")?;
        let time = schema.get_field("timestamp")?;
        let size = schema.get_field("size")?;
        let duration = schema.get_field("duration")?;

        let mut doc = Document::default();

        doc.add_text(title, self.title.clone());
        doc.add_text(transcript, self.transcript.clone());
        doc.add_text(desc, self.description.clone());
        doc.add_i64(id, self.id.get());
        doc.add_u64(size, self.file.size);
        doc.add_i64(duration, self.file.duration.into());

        let to_time = self.date.and_hms_opt(0, 0, 0).unwrap();
        let timestamp = DateTime::from_timestamp_secs(to_time.timestamp());
        doc.add_date(time, timestamp);

        for tag in self.tags.iter() {
            doc.add_text(tags, tag.to_string());
        }

        Ok(doc)
    }
}
