//! # Database
//!

use std::fmt::Debug;

use anyhow::Result;
use rand::distributions::{Alphanumeric, DistString};
use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{Field, NumericOptions, Schema, STORED, TEXT},
    DateOptions, DateTimePrecision, Document, Index, IndexBuilder, IndexReader, IndexWriter,
    ReloadPolicy, Searcher,
};

use crate::{
    entry::{Entry, Multiplied},
    INDEX, ROOT, SCHEMA, SEARCHER, WRITER,
};

pub struct Database {
    pub schema: Schema,
    index: Index,
    writer: IndexWriter,
    pub searcher: IndexReader,
}

impl Database {
    pub fn schema() -> Schema {
        let mut builder = Schema::builder();

        builder.add_text_field("title", TEXT | STORED);
        builder.add_text_field("text", TEXT | STORED);
        builder.add_text_field("tags", TEXT | STORED);

        let nums = NumericOptions::default()
            .set_indexed()
            .set_stored()
            .set_fast();

        builder.add_i64_field("id", nums);

        let dates = DateOptions::default()
            .set_stored()
            .set_indexed()
            .set_precision(DateTimePrecision::Seconds);
        builder.add_date_field("timestamp", dates);

        builder.build()
    }

    pub fn get_fields() -> Result<(Field, Field, Field)> {
        let schema = Self::schema();
        let title = schema.get_field("title")?;
        let text = schema.get_field("text")?;
        let tags = schema.get_field("tags")?;

        Ok((title, text, tags))
    }

    pub fn new() -> Result<Self> {
        let schema = Self::schema();
        let path = format!("{}/db/", ROOT.as_str());

        // TODO: Error handling
        let index = Index::open_in_dir(path)?;

        let writer = index.writer(25_000_000)?;

        let searcher = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        Ok(Self {
            schema,
            index,
            writer,
            searcher,
        })
    }

    pub fn random() -> Result<()> {
        let mut write = WRITER.lock().unwrap();
        for x in 25..110 {
            let mut e = Entry::default();
            e.text = format!(
                "Tantivy test: {}",
                Alphanumeric.sample_string(&mut rand::thread_rng(), 24)
            );
            e.title = format!("docs: {:#04x}", x);
            let d = e.to_document()?;
            let _ = write.add_document(d);
        }

        // WRITER.commit()?;
        let _ = write.commit()?;

        Ok(())
    }

    pub fn search(query: &str) -> Result<Vec<Entry>> {
        let schema = &SCHEMA;

        let title = schema.get_field("title")?;
        let text = schema.get_field("text")?;
        let tag = schema.get_field("tags")?;
        let _ts = schema.get_field("timestamp")?;
        let _id = schema.get_field("id")?;

        let searcher = SEARCHER.searcher();
        let parser = QueryParser::for_index(&INDEX, vec![title, text, tag]);
        let query = parser.parse_query(query)?;
        let docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        let mut entries = Vec::with_capacity(docs.len());

        for (_s, a) in docs {
            let doc = searcher.doc(a)?;
            let js = schema.to_json(&doc);
            let m = serde_json::from_str::<Multiplied>(&js)?;
            let e: Entry = m.try_into()?;
            entries.push(e);
        }

        Ok(entries)
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database").finish()
    }
}
