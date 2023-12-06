//! # Populate the list of files in the ingest folder

use crate::ROOT;

use super::Metadata;
use std::fs;

pub fn load() -> Vec<(String, Metadata)> {
    let path = format!("{}/ingest/", ROOT.as_str());
    let mut items = Vec::new();
    if let Ok(itr) = fs::read_dir(path.clone()) {
        for entry in itr.into_iter() {
            if entry.is_err() {
                continue;
            }
            let entry = entry.unwrap();
            let name = entry.file_name();
            if let Ok(i) = name.into_string() {
                let m = Metadata::new(format!("{}{}", path, i));
                if m.is_err() {
                    continue;
                }
                let m = m.unwrap();
                items.push((i, m));
            }
        }
    }

    items
}
