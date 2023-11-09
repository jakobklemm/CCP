//! Entry

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use polodb_core::bson::doc;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    key: Uuid,
    id: Id,
    title: String,
    // primary key fuckery
    tags: Vec<Uuid>,
    people: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    key: Uuid,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    key: Uuid,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id(i64);

impl Default for Entry {
    fn default() -> Self {
        Self {
            key: Uuid::new_v4(),
            id: Id::default(),
            title: String::from("TITLE MISSING"),
            tags: Vec::new(),
            people: Vec::new(),
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
        let config = col.find_one(doc! {"_id": "CONFIG"}).expect("Unable to find configuration in database.");
        println!("CONFIG: {:?}", config);
        let counter = config.unwrap().counter + 1;
        let _ = col.update_one(doc! {
            "_id": "CONFIG",
        }, doc! {
            "$set": doc! {
                "counter": counter
            }
        });
        Id(counter)
    }
}
