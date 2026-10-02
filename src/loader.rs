// src/loader.rs
use std::path::Path;
use std::fs;

/// Loads a PDF file from the given path and extracts its text content.
pub fn load_pdf<P: AsRef<Path>>(path: P) -> Result<String, String> {
    let path_ref = path.as_ref();

    // 1. Read the raw file bytes into memory, handling errors cleanly
    let bytes = fs::read(path_ref)
        .map_err(|e| format!("Failed to read PDF file: {}", e))?;

    // 2. Extract text from the loaded file bytes
    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| format!("Failed to extract text from PDF: {}", e))?;

    Ok(text)
}
