//! # Config

use serde::{Deserialize, Serialize};
use polodb_core::bson::doc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "_id")]
    id: String,
    globalCounter: i32,
    root: String,
}

impl Config {
    pub fn getNextID() -> i32 {
        let collection = crate::DATABASE.collection::<Config>("config");
        let config = Self::getConfig();
        match config {
            Some(c) => {
                println!("INNER CONFIG: {:?}", c);
                let new_counter = c.globalCounter + 1;
                // TODO: Update counter
                let _ = collection.update_one(doc! {
                    "_id": "CONFIG"
                }, doc! {
                    "$set": doc! {
                        "globalCounter": new_counter
                    }
                });
                return new_counter;
            }
            None => {
                // No config exists
                let new_config = Config::default();
                // TODO: Handle error
                let _ = collection.insert_one(new_config);
                return 0;
            }
        }
    }

    pub fn getRoot() -> String {
        let config = Self::getConfig();
        match config {
            Some(c) => {
                return c.root;
            }
            None => {
                let new_config = Config::default();
                let collection = crate::DATABASE.collection::<Config>("config");
                let _ = collection.insert_one(new_config.clone());
                return new_config.root;
            }
        }
    }

    fn getConfig() -> Option<Config> {
        let collection = crate::DATABASE.collection::<Config>("config");
        let config = collection.find_one(doc! {
            "_id": "CONFIG"
        }).unwrap();
        return config;
    }
}

impl Default for Config {
    fn default() -> Self {
        let dir: String = match std::env::var("DCP_ROOT") {
            Ok(path) => path,
            Err(_e) => {
                String::from("/database")
            }
        };
        Self { id: "CONFIG".to_string(), globalCounter: 0, root: dir }
    }
}
