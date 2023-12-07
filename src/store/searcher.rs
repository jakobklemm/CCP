//! # Searcher
//!

use std::{
    fs,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{NumericOptions, Schema, INDEXED, STORED, TEXT},
    DateOptions, DateTimePrecision, Document, Index, IndexReader, IndexWriter, ReloadPolicy,
};

use crate::ROOT;

pub struct Searcher {
    pub schema: Schema,
    index: Index,
    writer: Arc<Mutex<IndexWriter>>,
    reader: IndexReader,
}

impl Searcher {
    pub fn new() -> Result<Self> {
        let schema = Self::schema();
        let path = format!("{}/store/search/", ROOT.as_str());
        let _ = fs::create_dir_all(path.clone());
        let index = match Index::create_in_dir(path.clone(), schema.to_owned()) {
            Ok(i) => i,
            Err(_) => Index::open_in_dir(path).expect("Unable to open Tantivy Database"),
        };
        let writer = Arc::new(Mutex::new(index.writer(25_000_000)?));
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        Ok(Self {
            schema,
            index,
            writer,
            reader,
        })
    }

    pub fn index_one(&self, doc: Document) -> Result<()> {
        let mut writer = self
            .writer
            .lock()
            .expect("Unrecoverable internal system error.");

        let _ = writer.add_document(doc)?;

        let _ = writer.commit()?;

        Ok(())
    }

    pub fn index_many(&self, docs: Vec<Document>) -> Result<()> {
        let mut writer = self
            .writer
            .lock()
            .expect("Unrecoverable internal system error.");

        for doc in docs {
            let _ = writer.add_document(doc)?;
        }

        let _ = writer.commit()?;

        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<Document>> {
        let title = self.schema.get_field("title")?;
        let text = self.schema.get_field("transcript")?;
        let desc = self.schema.get_field("description")?;
        let tags = self.schema.get_field("tags")?;
        let _ts = self.schema.get_field("timestamp")?;
        let _id = self.schema.get_field("id")?;

        // println!("Searching!");

        let searcher = self.reader.searcher();

        let mut parser = QueryParser::for_index(&self.index, vec![title, text, tags]);
        parser.set_conjunction_by_default();
        parser.set_field_fuzzy(title, false, 2, false);
        parser.set_field_fuzzy(text, false, 2, false);
        parser.set_field_fuzzy(desc, false, 2, false);
        let query = parser.parse_query(query)?;

        let docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut found = Vec::with_capacity(docs.len());
        for (_s, addr) in docs {
            if let Ok(doc) = searcher.doc(addr) {
                found.push(doc);
            }
        }

        Ok(found)
    }

    pub fn schema() -> Schema {
        let mut builder = Schema::builder();

        builder.add_text_field("title", TEXT);
        builder.add_text_field("description", TEXT);
        builder.add_text_field("transcript", TEXT);
        builder.add_text_field("tags", TEXT);

        let nums = NumericOptions::default()
            .set_indexed()
            .set_stored()
            .set_fast();

        let notstored = NumericOptions::default().set_indexed().set_fast();

        builder.add_i64_field("id", nums);

        builder.add_f64_field("size", notstored.clone());
        builder.add_f64_field("duration", notstored);

        let dates = DateOptions::default()
            .set_indexed()
            .set_precision(DateTimePrecision::Seconds);
        builder.add_date_field("timestamp", dates);

        builder.build()
    }
}
