use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Extract text page-by-page and streams it straight to the file handle
pub fn stream_pdf_to_file<P: AsRef<Path>>(source_path: P, dest_file: &mut File) -> Result<(), String> {
    let path_ref = source_path.as_ref();

    let pages = pdf_extract::extract_text_by_pages(path_ref)
        .map_err(|e| format!("Failed to extract pages from PDF: {}", e))?;

    for (idx, page_text) in pages.iter().enumerate() {
        let page_num = idx + 1;
        
        dest_file
            .write_all(page_text.as_bytes())
            .map_err(|e| format!("Failed to write page {} to disk: {}", page_num, e))?;
        
        println!("   -> Streamed page {} successfully", page_num);
    }

    dest_file.flush().map_err(|e| format!("Flush error: {}", e))?;

    Ok(())
}
