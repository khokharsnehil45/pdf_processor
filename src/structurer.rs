use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents a single structured page payload optimized for AI LLM ingestion and vector embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagePayload {
    pub page_number: u32,
    pub character_count: usize,
    pub word_count: usize,
    pub text_payload: String,
}

/// Metadata describing the source document and processing statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub source_file: String,
    pub total_pages: usize,
    pub total_characters: usize,
    pub total_words: usize,
    pub extraction_timestamp: String,
}

/// The top-level structured document envelope containing document-wide metadata
/// and an ordered array of page payloads ready for RAG pipelines or vector stores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredDocument {
    pub metadata: DocumentMetadata,
    pub pages: Vec<PagePayload>,
}

impl PagePayload {
    /// Creates a structured page payload with computed character and word counts.
    pub fn new(page_number: u32, raw_text: String) -> Self {
        let character_count = raw_text.chars().count();
        let word_count = raw_text.split_whitespace().count();
        Self {
            page_number,
            character_count,
            word_count,
            text_payload: raw_text,
        }
    }
}

impl StructuredDocument {
    /// Constructs a structured document from extracted page tuples `(page_num, text)`.
    pub fn from_extracted_pages<P: AsRef<Path>>(source_path: P, extracted_pages: Vec<(u32, String)>) -> Self {
        let file_name = source_path
            .as_ref()
            .file_name()
            .map(|f| f.to_string_lossy().into_owned())
            .unwrap_or_else(|| source_path.as_ref().to_string_lossy().into_owned());

        let mut total_characters = 0;
        let mut total_words = 0;

        let pages: Vec<PagePayload> = extracted_pages
            .into_iter()
            .map(|(page_num, text)| {
                let page = PagePayload::new(page_num, text);
                total_characters += page.character_count;
                total_words += page.word_count;
                page
            })
            .collect();

        let total_pages = pages.len();
        let extraction_timestamp = Utc::now().to_rfc3339();

        Self {
            metadata: DocumentMetadata {
                source_file: file_name,
                total_pages,
                total_characters,
                total_words,
                extraction_timestamp,
            },
            pages,
        }
    }

    /// Serializes the structured document into formatted (pretty-printed) JSON.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serializes the structured document into compact JSON.
    #[allow(dead_code)]
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serializes each page payload as a JSON Lines (JSONL) entry.
    /// Useful for direct batch indexing into vector databases (e.g. Pinecone, Qdrant, Chroma).
    pub fn to_jsonl(&self) -> Result<String, serde_json::Error> {
        let mut output = String::new();
        for page in &self.pages {
            let line = serde_json::to_string(page)?;
            output.push_str(&line);
            output.push('\n');
        }
        Ok(output)
    }
}
