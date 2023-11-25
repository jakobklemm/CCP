//! Entry

use crate::{config::Config, database::Database, SCHEMA};
use anyhow::Result;
use chrono::{DateTime as CDT, Local};
use polodb_core::bson::doc;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tantivy::{schema::Schema, DateTime, Document};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: Id,
    pub title: String,
    pub timestamp: CDT<Local>,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multiplied {
    id: Vec<i64>,
    title: Vec<String>,
    timestamp: Vec<String>,
    text: Vec<String>,
    tags: Option<Vec<String>>,
}

use anyhow::Error;

impl TryInto<Entry> for Multiplied {
    type Error = Error;

    fn try_into(self) -> std::prelude::v1::Result<Entry, Self::Error> {
        // TODO: Memory & CPU efficiency
        let mut id = 0;
        let mut title = String::default();
        let mut timestamp = Local::now();
        let mut text = String::default();

        for i in self.id {
            id = i;
            break;
        }

        for t in self.title {
            title = t;
            break;
        }

        for t in self.timestamp {
            timestamp = t.parse::<CDT<Local>>()?;
            break;
        }

        for t in self.text {
            text = t;
            break;
        }

        Ok(Entry {
            id: Id(id),
            title,
            timestamp,
            text,
            tags: self.tags.unwrap_or(Vec::default()),
        })
    }
}

impl Entry {
    pub fn new(title: impl ToString) -> Self {
        let mut e = Entry::default();
        e.title = title.to_string();
        e
    }

    pub fn to_document(self) -> Result<Document> {
        let mut doc = Document::default();

        let schema = &SCHEMA;

        let id = schema.get_field("id")?;
        let title = schema.get_field("title")?;
        let text = schema.get_field("text")?;
        let tags = schema.get_field("tags")?;
        let timestamp = schema.get_field("timestamp")?;

        doc.add_text(title, self.title);
        doc.add_text(text, self.text);
        doc.add_i64(id, self.id.0);
        doc.add_date(
            timestamp,
            DateTime::from_timestamp_secs(self.timestamp.timestamp()),
        );

        for tag in self.tags {
            doc.add_text(tags, tag);
        }

        Ok(doc)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Id(pub i64);

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: Id::default(),
            title: String::from("TITLE MISSING"),
            timestamp: Local::now(),
            tags: Vec::new(),
            text: String::new(),
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::get_inc()
    }
}

use crate::ROOT;
use std::fs;

impl Id {
    pub fn temp_path(&self) -> Result<String> {
        fs::create_dir_all(format!("{}/temp/{}/", ROOT.as_str(), self.0))?;
        Ok(format!("{}temp/{}/pass_1.mp4", ROOT.as_str(), self.0))
    }

    pub fn temp_dir(&self) -> String {
        format!("{}temp/{}/", ROOT.as_str(), self.0)
    }

    pub fn transcript(&self) -> String {
        format!("{}temp/{}/pass_1.txt", ROOT.as_str(), self.0)
    }

    pub fn out_path(&self) -> Result<String> {
        let s = self.0 / 100;
        fs::create_dir_all(format!("{}/store/{}/", ROOT.as_str(), s))?;
        Ok(format!("{}/store/{}/{}.mp4", ROOT.as_str(), s, self.0))
    }

    fn get_inc() -> Self {
        let col = crate::DATABASE.collection::<Config>("config");
        let config = col
            .find_one(doc! {"_id": "CONFIG"})
            .expect("Unable to find configuration in database.");
        let counter = config.unwrap().counter + 1;
        let _ = col.update_one(
            doc! {
                "_id": "CONFIG",
            },
            doc! {
                "$set": doc! {
                    "counter": counter
                }
            },
        );
        Id(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_id_json() {
        let id = Id::default();
        let s = serde_json::to_string(&id).unwrap();
        let e: Id = serde_json::from_str(&s).unwrap();
    }
}
