//! # DocStore

use anyhow::{anyhow, Result};
use polodb_core::{bson::Document, ClientCursor, Database};
use serde::{de::DeserializeOwned, Serialize};

use crate::ROOT;

use super::Entity;

pub struct DocStore {
    pub database: Database,
}

impl DocStore {
    pub fn new() -> Result<Self> {
        let path = format!("{}/store/CCP.db", ROOT.as_str());
        Ok(Self {
            database: Database::open_file(path)?,
        })
    }

    pub fn insert<E: Entity>(&self, item: E) -> Result<()> {
        let col = self.database.collection::<E>(E::collection());
        let _ = col.insert_one(item)?;

        Ok(())
    }

    pub fn get_one<E: Entity + DeserializeOwned>(&self, query: Document) -> Result<E> {
        let col = self.database.collection::<E>(E::collection());
        if let Some(e) = col.find_one(query)? {
            Ok(e)
        } else {
            Err(anyhow!("Unable to find document"))
        }
    }

    pub fn get_many<E: Entity + DeserializeOwned>(
        &self,
        query: Document,
    ) -> Result<ClientCursor<E>> {
        let col = self.database.collection::<E>(E::collection());
        let res = col.find(query)?;
        Ok(res)
    }

    pub fn get_all<E: Entity + DeserializeOwned>(&self) -> Result<ClientCursor<E>> {
        let col = self.database.collection::<E>(E::collection());
        let found = col.find(None)?;
        Ok(found)
    }

    pub fn update_one<E: Entity>(&self, query: Document, changes: Document) -> Result<()> {
        let col = self.database.collection::<E>(E::collection());
        let _ = col.update_one(query, changes)?;

        Ok(())
    }
}
