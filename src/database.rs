//! # Database
//!

use anyhow::Result;
use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{Schema, STORED, TEXT},
    Document, Index, IndexBuilder, IndexReader, IndexWriter, ReloadPolicy, Searcher,
};

use crate::ROOT;

pub struct Database {
    schema: Schema,
    index: Index,
    writer: IndexWriter,
    pub searcher: IndexReader,
}

impl Database {
    pub fn schema() -> Schema {
        let mut builder = Schema::builder();

        builder.add_text_field("title", TEXT | STORED);
        builder.add_text_field("text", TEXT);
        builder.add_text_field("tag", TEXT);

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

    pub fn random(&mut self) -> Result<()> {
        let title = self.schema.get_field("title")?;
        let text = self.schema.get_field("text")?;
        let tag = self.schema.get_field("tag")?;

        for x in 1..100 {
            let mut doc = Document::default();
            doc.add_text(title, format!("Document: {}", x));
            doc.add_text(text, format!("some text stuff: {}", (x * 184 + 53) % 54));
            for y in 534..538 {
                doc.add_text(tag, format!("{:#06x}", y % 122));
            }
            let _ = self.writer.add_document(doc);
        }

        self.writer.commit()?;

        Ok(())
    }

    pub fn search(&mut self, query: impl ToString) -> Result<()> {
        let title = self.schema.get_field("title")?;
        let text = self.schema.get_field("text")?;
        let tag = self.schema.get_field("tag")?;

        let searcher = self.searcher.searcher();
        let parser = QueryParser::for_index(&self.index, vec![title, text, tag]);
        let query = parser.parse_query(&query.to_string())?;
        let docs = searcher.search(&query, &TopDocs::with_limit(25))?;

        for (s, a) in docs {
            let doc = searcher.doc(a)?;
            println!("{}: {}", s, self.schema.to_json(&doc));
        }

        Ok(())
    }
}
