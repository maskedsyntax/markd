use anyhow::Result;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, doc};
use std::path::Path;

pub struct MarkdIndexer {
    pub index: Index,
    pub schema: Schema,
    pub title: Field,
    pub body: Field,
    pub path: Field,
}

impl MarkdIndexer {
    pub fn new(index_path: &Path) -> Result<Self> {
        let mut schema_builder = Schema::builder();
        let title = schema_builder.add_text_field("title", TEXT | STORED);
        let body = schema_builder.add_text_field("body", TEXT);
        let path = schema_builder.add_text_field("path", STORED);
        let schema = schema_builder.build();

        if !index_path.exists() {
            std::fs::create_dir_all(index_path)?;
        }

        let index = Index::open_or_create(tantivy::directory::MmapDirectory::open(index_path)?, schema.clone())?;
        
        Ok(Self { index, schema, title, body, path })
    }

    pub fn index_document(&self, title: &str, body: &str, path: &str) -> Result<()> {
        let mut index_writer: IndexWriter = self.index.writer(50_000_000)?;
        index_writer.add_document(doc!(
            self.title => title,
            self.body => body,
            self.path => path,
        ))?;
        index_writer.commit()?;
        Ok(())
    }
}
