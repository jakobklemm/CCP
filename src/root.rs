//! # Root

use std::fs::{self, File};

use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::ROOT;

lazy_static! {
    static ref PATH: String = format!("{}/ccp.json", ROOT.as_str());
}

static ERR: &'static str = "Unable to create default configuration.";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Root {
    counter: i64,
    /// (count, tag)
    tags: Vec<(usize, String)>,
}

impl Root {
    fn from_file() -> Result<Self> {
        let f = fs::read_to_string(PATH.as_str())?;

        let root: Root = serde_json::from_str(&f)?;

        Ok(root)
    }

    fn exists() -> bool {
        let f = File::open(PATH.as_str());
        if f.is_ok() {
            true
        } else {
            false
        }
    }

    fn write_file(&self) -> Result<()> {
        let s = serde_json::to_string_pretty(&self)?;
        if !Self::exists() {
            let _ = Self::default();
        }

        let _ = fs::write(PATH.as_str(), s)?;
        Ok(())
    }

    pub fn get_tags(&self) -> Vec<String> {
        let mut ntags = self.tags.clone();
        ntags.sort_by(|(xa, _xb), (ya, _yb)| xa.partial_cmp(ya).unwrap());
        ntags.into_iter().map(|(_x, y)| y).collect()
    }
}

impl Default for Root {
    fn default() -> Self {
        if let Ok(r) = Self::from_file() {
            r
        } else {
            // 1. Check if file exists
            if Self::exists() {
                panic!("Root database exists but isn't valid!");
            }
            let r = Self {
                counter: 0,
                tags: Vec::default(),
            };
            let s = serde_json::to_string_pretty(&r).expect(ERR);

            let _ = fs::write(PATH.as_str(), &s).expect(ERR);

            r
        }
    }
}
