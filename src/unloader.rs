use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::structurer::StructuredDocument;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    JsonLines,
    PlainText,
}

impl OutputFormat {
    pub fn description(&self) -> &'static str {
        match self {
            OutputFormat::Json => "Structured JSON (Pretty)",
            OutputFormat::JsonLines => "JSON Lines (Vector DB ingest ready)",
            OutputFormat::PlainText => "Raw Plain Text",
        }
    }
}

pub struct SaveResult {
    pub saved_path: String,
    pub bytes_written: usize,
    pub format: OutputFormat,
}

/// Safely prepares and creates the target file handle on disk.
#[allow(dead_code)]
pub fn prepare_destination_file(output_path: &str) -> Result<(File, String), String> {
    let final_path_str = if output_path.trim().is_empty() {
        "output.json"
    } else {
        output_path.trim()
    };

    let path = Path::new(final_path_str);

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            if let Err(err) = fs::create_dir_all(parent) {
                return Err(format!("Failed to create directories: {}", err));
            }
        }
    }

    match File::create(path) {
        Ok(file_handle) => Ok((file_handle, final_path_str.to_string())),
        Err(err) => Err(format!("Failed to create file '{}': {}", final_path_str, err)),
    }
}

/// Saves the structured document into the destination file, formatting as
/// structured JSON (.json), JSON Lines (.jsonl), or plain text (.txt) automatically.
pub fn save_structured_document(
    output_path: &str,
    document: &StructuredDocument,
) -> Result<SaveResult, String> {
    let final_path_str = if output_path.trim().is_empty() {
        "output.json"
    } else {
        output_path.trim()
    };

    let path = Path::new(final_path_str);

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            if let Err(err) = fs::create_dir_all(parent) {
                return Err(format!("Failed to create directories: {}", err));
            }
        }
    }

    let format = if final_path_str.ends_with(".jsonl") {
        OutputFormat::JsonLines
    } else if final_path_str.ends_with(".txt") {
        OutputFormat::PlainText
    } else {
        OutputFormat::Json
    };

    let mut file = File::create(path)
        .map_err(|err| format!("Failed to create file '{}': {}", final_path_str, err))?;

    let bytes_written = match format {
        OutputFormat::Json => {
            let json_str = document
                .to_json_pretty()
                .map_err(|e| format!("JSON serialization error: {}", e))?;
            file.write_all(json_str.as_bytes())
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            json_str.len()
        }
        OutputFormat::JsonLines => {
            let jsonl_str = document
                .to_jsonl()
                .map_err(|e| format!("JSONL serialization error: {}", e))?;
            file.write_all(jsonl_str.as_bytes())
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            jsonl_str.len()
        }
        OutputFormat::PlainText => {
            let mut total = 0;
            for page in &document.pages {
                let bytes = page.text_payload.as_bytes();
                file.write_all(bytes)
                    .map_err(|e| format!("Failed to write to file: {}", e))?;
                total += bytes.len();
            }
            total
        }
    };

    file.flush().map_err(|e| format!("Flush error: {}", e))?;

    Ok(SaveResult {
        saved_path: final_path_str.to_string(),
        bytes_written,
        format,
    })
}
