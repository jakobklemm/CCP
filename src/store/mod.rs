//! # Store
//!

mod config;
mod docstore;
mod item;
mod searcher;

use anyhow::Result;
use config::Config;
use docstore::DocStore;
use item::Item;
use polodb_core::bson::{doc, Document};
use searcher::Searcher;
use serde::de::DeserializeOwned;
use tantivy::Document as FTSDoc;

pub struct Database {
    searcher: Searcher,
    documents: DocStore,
}

impl Database {
    pub fn new() -> Result<Self> {
        let searcher = Searcher::new()?;
        let documents = DocStore::new()?;
        Ok(Self {
            searcher,
            documents,
        })
    }

    pub fn get_config(&self) -> Result<Config> {
        self.documents.get_one::<Config>(doc! {"_id": "CONFIG"})
    }

    pub fn get_config_incr(&self) -> Result<i64> {
        let conf = self.documents.get_one::<Config>(doc! {"_id": "CONFIG"})?;
        let counter = conf.get_counter() + 1;
        let _ = self.documents.update_one::<Config>(
            doc! {"_id": "CONFIG"},
            doc! {"$set": doc! {"counter": counter}},
        )?;

        Ok(counter)
    }

    pub fn insert_indexed<E: Item>(&self, item: E) -> Result<()> {
        let _ = self.documents.insert(item.clone())?;
        self.searcher.index_one(item.to_document()?)
    }

    pub fn insert_indexed_many<E: Item>(&self, items: Vec<E>) -> Result<()> {
        for item in &items {
            let _ = self.documents.insert(item.clone())?;
        }
        self.searcher.index_many(
            items
                .iter()
                .map(|x| x.to_document())
                .collect::<Result<Vec<FTSDoc>>>()?,
        )
    }

    pub fn search(&self, query: &str) -> Result<Vec<FTSDoc>> {
        self.searcher.search(query)
    }

    pub fn insert<E: Item>(&self, item: E) -> Result<()> {
        self.documents.insert(item)
    }

    pub fn get_one<E: Item + DeserializeOwned>(&self, query: Document) -> Result<E> {
        self.documents.get_one(query)
    }

    pub fn get_many<E: Item + DeserializeOwned>(&self, query: Document) -> Result<Vec<E>> {
        self.documents.get_many(query)
    }

    pub fn get_all<E: Item + DeserializeOwned>(&self) -> Result<Vec<E>> {
        self.documents.get_all()
    }

    pub fn update_one<E: Item>(&self, query: Document, changes: Document) -> Result<()> {
        self.documents.update_one::<E>(query, changes)
    }
}
