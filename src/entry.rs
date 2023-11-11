//! Entry

use crate::config::Config;
use polodb_core::bson::doc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub key: Uuid,
    pub id: Id,
    pub title: String,
    // primary key fuckery
    pub tags: Vec<Uuid>,
    people: Vec<Uuid>,
    text: String,
}

impl Entry {
    pub fn new(title: impl ToString) -> Self {
        let mut e = Entry::default();
        e.title = title.to_string();
        e
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    key: Uuid,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    key: Uuid,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id(pub i64);

impl Default for Entry {
    fn default() -> Self {
        Self {
            key: Uuid::new_v4(),
            id: Id::default(),
            title: String::from("TITLE MISSING"),
            tags: Vec::new(),
            people: Vec::new(),
            text: String::new(),
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::get_inc()
    }
}

impl Id {
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
