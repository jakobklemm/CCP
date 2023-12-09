//! # Id

use std::{fs};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{DATABASE, ROOT};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Id(i64);

impl Default for Id {
    fn default() -> Self {
        Self(
            DATABASE
                .get_config_incr()
                .expect("Unable to parse configuration!"),
        )
    }
}

impl Id {
    pub fn get(&self) -> i64 {
        self.0
    }

    pub fn temp_dir(&self) -> Result<String> {
        let path = format!("{}/temp/{}/", ROOT.as_str(), self.0);

        fs::create_dir_all(&path)?;

        Ok(path)
    }

    pub fn temp_path(&self) -> Result<String> {
        Ok(format!("{}pass_1.mp4", self.temp_dir()?))
    }

    pub fn text_path(&self) -> Result<String> {
        Ok(format!("{}pass_1.txt", self.temp_dir()?))
    }

    pub fn srt_path(&self) -> Result<String> {
        Ok(format!("{}pass_1.srt", self.temp_dir()?))
    }

    pub fn srt_out(&self) -> Result<String> {
        let path = format!("{}/subs/{}/", ROOT.as_str(), self.get_s());
        let last = format!("{}.srt", self.get());
        fs::create_dir_all(path.clone())?;

        Ok(path + &last)
    }

    pub fn meta_path(&self) -> Result<String> {
        let path = format!("{}/meta/{}/", ROOT.as_str(), self.get_s());
        let last = format!("{}.json", self.get());
        fs::create_dir_all(&path)?;
        Ok(path + &last)
    }

    pub fn data_path(&self) -> Result<String> {
        let path = format!("{}/data/{}/", ROOT.as_str(), self.get_s());
        let last = format!("{}.mp4", self.get());
        fs::create_dir_all(&path)?;
        Ok(path + &last)
    }

    fn get_s(&self) -> i64 {
        self.0 / 100
    }
}
