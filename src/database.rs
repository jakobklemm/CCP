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
    query::{BooleanQuery, FuzzyTermQuery, QueryClone, QueryParser},
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
            e.text = format!("Tantivy test: {}", lipsum_words_with_rng(thread_rng(), x));
            e.title = format!("documents {}", lipsum_words_with_rng(thread_rng(), 8));
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
        let tags = schema.get_field("tags")?;
        let _ts = schema.get_field("timestamp")?;
        let _id = schema.get_field("id")?;

        // println!("Searching!");

        let searcher = SEARCHER.searcher();

        let parser = QueryParser::for_index(&INDEX, vec![title, text, tags]);
        let query_all = parser.parse_query(query)?;

        let title_term = Term::from_field_text(title, query);
        let title_query = FuzzyTermQuery::new(title_term, 2, true);

        let text_term = Term::from_field_text(text, query);
        let text_query = FuzzyTermQuery::new(text_term, 2, true);

        let tags_term = Term::from_field_text(tags, query);
        let tags_query = FuzzyTermQuery::new(tags_term, 2, true);

        let comps = vec![
            (Occur::Should, title_query.box_clone()),
            (Occur::Should, text_query.box_clone()),
            (Occur::Should, tags_query.box_clone()),
        ];
        let query = BooleanQuery::new(comps);

        let docs = searcher.search(&query_all, &TopDocs::with_limit(100))?;

        let mut entries = Vec::with_capacity(docs.len());

        for (_s, a) in docs {
            let doc = searcher.doc(a)?;
            let js = schema.to_json(&doc);
            // println!("{} - {:?}", s, js);
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
