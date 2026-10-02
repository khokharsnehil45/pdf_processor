// src/unloader.rs
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Saves the extracted text content to the specified destination path.
pub fn unload_text<P: AsRef<Path>>(content: &str, destination: P) -> Result<(), String> {
    let path = destination.as_ref();

    // If the user specified a directory path that doesn't exist, create it
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directories: {}", e))?;
        }
    }

    // Create or truncate the target file
    let mut file = File::create(path)
        .map_err(|e| format!("Failed to create destination file: {}", e))?;

    // Write the contents
    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write data to file: {}", e))?;

    Ok(())
}
