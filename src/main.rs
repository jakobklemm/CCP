// Main

use anyhow::Error;

mod entry;
mod processor;
mod config;

use config::Config;
use entry::{Entry};
use lazy_static::lazy_static;
use polodb_core::{Database, bson::doc};

lazy_static! {
    pub static ref DATABASE: Database = {
        let root = std::env::var("CCP_ROOT").unwrap_or("/database".to_string());
        Database::open_file(format!("{}/ccp-polo.db", root)).expect("Unable to open database")
    };
    pub static ref ROOT: String = std::env::var("CCP_ROOT").unwrap_or("/database".to_string());
}

fn main() -> Result<(), Error> {
    ensure_configured();
    let e = Entry::default();
    println!("{:?}", e);
    Ok(())
}

fn ensure_configured() {
    let col = DATABASE.collection::<Config>("config");
    let config = col.find_one(doc! {
        "_id": "CONFIG"
    }).expect("Unable to properly handle configuration.");
    match config {
        Some(c) => {
            println!("Existing configuration found: {:?}", c);
        }
        None => {
            println!("No existing configuration found, creating new one.");
            let config = Config::default();
            let _ = col.insert_one(config);
        }
    }
}
