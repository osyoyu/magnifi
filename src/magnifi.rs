use std::path::PathBuf;

use tantivy::{Index, Document, DocAddress, Score};
use tantivy::schema::{Field, FieldType};
use tantivy::query::QueryParser;
use tantivy::collector;
use tantivy_tokenizer_tiny_segmenter::tokenizer::TinySegmenterTokenizer;

#[derive(Clone)]
pub struct MagnifiApp;

impl MagnifiApp {
    pub fn index(&self) -> tantivy::Index {
        let index_path = "../tantivy-jawp2";
        let index = Index::open_in_dir(PathBuf::from(index_path)).expect(format!("Failed to open index in {}", index_path).as_str());
        index.tokenizers().register("tinyseg", TinySegmenterTokenizer {});
        index
    }

    pub fn search(&self, query: String) -> Vec<(Score, Document)> {
        let index = self.index();
        let schema = index.schema();

        let default_fields: Vec<Field> = schema
            .fields()
            .iter()
            .enumerate()
            .filter(|&(_, ref field_entry)|
                match *field_entry.field_type() {
                    FieldType::Str(ref text_field_options) => {
                        text_field_options.get_indexing_options().is_some()
                    },
                    _ => false,
                }
            )
            .map(|(i, _)| Field(i as u32))
            .collect();

        let query_parser = QueryParser::new(schema.clone(), default_fields, index.tokenizers().clone());

        let query = query_parser.parse_query(&query).expect("Query parsing failed");
        let reader = index.reader().expect("Couldn't open reader");
        let searcher = reader.searcher();

        let (top_docs, _doc_count): (Vec<(Score, DocAddress)>, usize) = searcher.search(&query, &(collector::TopDocs::with_limit(10), collector::Count)).unwrap();

        let mut docs: Vec<(Score, Document)> = vec![];
        for doc_hit in top_docs {
            docs.push((doc_hit.0, searcher.doc(doc_hit.1).unwrap()));
        }

        docs
    }
}
