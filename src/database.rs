//! # Database
//!

use std::fmt::Debug;

use anyhow::Result;
use lipsum::{lipsum_title, lipsum_words_with_rng};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, FuzzyTermQuery, QueryClone, QueryParser, RegexQuery},
    query_grammar::Occur,
    schema::{Field, NumericOptions, Schema, TextOptions, INDEXED, STORED, TEXT},
    DateOptions, DateTimePrecision, Document, Index, IndexBuilder, IndexReader, IndexWriter,
    ReloadPolicy, Searcher, Term,
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

        builder.add_i64_field("id", nums.clone());

        builder.add_f64_field("size", nums.clone());
        builder.add_f64_field("duration", nums);

        let dates = DateOptions::default()
            .set_stored()
            .set_indexed()
            .set_precision(DateTimePrecision::Seconds);
        builder.add_date_field("timestamp", dates);

        builder.build()
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
            e.text = format!("{}", lipsum_words_with_rng(thread_rng(), x));
            e.title = format!("{}", lipsum_words_with_rng(thread_rng(), 8));
            let d = e.to_document().unwrap();
            let e = write.add_document(d).unwrap();
        }

        // WRITER.commit()?;
        let _ = write.commit()?;

        Ok(())
    }

    pub fn search(query: &str) -> Result<Vec<Entry>> {
        let schema = &SCHEMA;

        let title = schema.get_field("title")?;
        let text = schema.get_field("text")?;
        let tags = schema.get_field("tags")?;
        let _ts = schema.get_field("timestamp")?;
        let _id = schema.get_field("id")?;

        // println!("Searching!");

        let searcher = SEARCHER.searcher();

        let mut parser = QueryParser::for_index(&INDEX, vec![title, text, tags]);
        parser.set_conjunction_by_default();
        parser.set_field_fuzzy(title, false, 2, false);
        parser.set_field_fuzzy(text, false, 2, false);
        let query = parser.parse_query(query)?;

        let docs = searcher.search(&query, &TopDocs::with_limit(55))?;

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
