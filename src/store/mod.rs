//! # Store
//!

mod config;
mod docstore;
mod searcher;

mod entity;
pub use entity::Entity;

use anyhow::Result;
use config::Config;
use docstore::DocStore;
use polodb_core::bson::{doc, Document};
use searcher::Searcher;
use serde::de::DeserializeOwned;
use tantivy::{schema::Schema, Document as FTSDoc};

pub struct Database {
    searcher: Searcher,
    documents: DocStore,
}

impl Database {
    pub fn new() -> Result<Self> {
        let searcher = Searcher::new()?;
        let documents = DocStore::new()?;
        let slf = Self {
            searcher,
            documents,
        };
        let _ = slf.ensure_configured()?;
        Ok(slf)
    }

    fn ensure_configured(&self) -> Result<Config> {
        match self.documents.get_one::<Config>(doc! {"_id": "CONFIG"}) {
            Ok(conf) => Ok(conf),
            Err(_) => {
                let conf = Config::default();
                let _ = self.insert(conf.clone())?;
                Ok(conf)
            }
        }
    }

    pub fn get_schema(&self) -> Schema {
        self::Searcher::schema()
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

    pub fn insert_indexed<E: Entity>(&self, item: E) -> Result<()> {
        let _ = self.documents.insert(item.clone())?;
        self.searcher
            .index_one(item.to_document(&self.searcher.schema)?)
    }

    pub fn insert_indexed_many<E: Entity>(&self, items: Vec<E>) -> Result<()> {
        for item in &items {
            let _ = self.documents.insert(item.clone())?;
        }
        self.searcher.index_many(
            items
                .iter()
                .map(|x| x.to_document(&self.searcher.schema))
                .collect::<Result<Vec<FTSDoc>>>()?,
        )
    }

    pub fn search<F: TryFrom<FTSDoc>>(&self, query: &str) -> Result<Vec<F>> {
        // TODO: Parameterize
        let res: Vec<F> = self
            .searcher
            .search(query, 100)?
            .into_iter()
            .map(|x| TryInto::<F>::try_into(x))
            .filter_map(Result::ok)
            .collect();

        Ok(res)
    }

    pub fn insert<E: Entity>(&self, item: E) -> Result<()> {
        self.documents.insert(item)
    }

    pub fn get_one<E: Entity + DeserializeOwned>(&self, query: Document) -> Result<E> {
        self.documents.get_one(query)
    }

    pub fn get_many<E: Entity + DeserializeOwned>(&self, query: Document) -> Result<Vec<E>> {
        self.documents.get_many(query)
    }

    pub fn get_all<E: Entity + DeserializeOwned>(&self) -> Result<Vec<E>> {
        self.documents.get_all()
    }

    pub fn update_one<E: Entity>(&self, query: Document, changes: Document) -> Result<()> {
        self.documents.update_one::<E>(query, changes)
    }
}
