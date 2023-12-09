//! # Job

use anyhow::{anyhow, Result};
use chrono::NaiveDate;

use crate::{application::tag::Tag, store::Entity, DATABASE};
use serde::{Deserialize, Serialize};

use super::timestamp::Timestamp;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    file: String,
    start: Timestamp,
    date: NaiveDate,
    end: Timestamp,
    title: String,
    language: Language,
    description: String,
    tags: Vec<Tag>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Language {
    EN,
    DE,
}

impl Language {
    pub fn from_str(s: &str) -> Language {
        let s = s.trim();
        match s {
            "en" => Self::EN,
            _ => Self::DE,
        }
    }

    pub fn from_input(lines: &[String]) -> Language {
        for l in lines {
            return Self::from_str(l);
        }
        Self::default()
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::EN => String::from("German"),
            Language::DE => String::from("Language"),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::DE
    }
}

impl Job {
    pub fn new(
        file: String,
        start_ts: &[String],
        end_ts: &[String],
        title_vec: &[String],
        desc_vec: &[String],
        lang_vec: &[String],
        date_input: &[String],
        tags: &[String],
    ) -> Result<Self> {
        let file = {
            if file == "" {
                return Err(anyhow!("invalid file selection"));
            }
            file
        };

        let start = Timestamp::from_input(start_ts)?;
        let end = Timestamp::from_input(end_ts)?;
        let title = {
            if let Some(l) = title_vec.first() {
                l.clone()
            } else {
                String::from("")
            }
        };
        let description = {
            if let Some(l) = desc_vec.first() {
                l.clone()
            } else {
                String::from("")
            }
        };
        let language = Language::from_input(lang_vec);
        let mut parsed_tags = Vec::new();
        for tgs in tags {
            // First line is all tags
            let tag = tgs.split(" ").collect::<Vec<&str>>();
            for t in tag {
                let created_tag = Tag::new(t)?;
                parsed_tags.push(created_tag);
            }
        }
        let date = {
            if let Some(d) = date_input.first() {
                NaiveDate::parse_from_str(&d, "%d-%m-%Y")?
            } else {
                return Err(anyhow!("invalid timestamp"));
            }
        };

        let job = Self {
            file,
            start,
            date,
            end,
            title,
            description,
            language,
            tags: parsed_tags,
        };

        let _ = DATABASE.insert(job.clone())?;
        Ok(job)
    }
}

impl Entity for Job {
    fn collection() -> &'static str {
        "jobs"
    }

    fn to_document(&self, _schema: &tantivy::schema::Schema) -> Result<tantivy::Document> {
        panic!("NOT SAFE FOR FTS STORE");
    }
}
