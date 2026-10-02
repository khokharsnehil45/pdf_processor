use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;

pub struct ProcessStats {
    pub total_pages: usize,
    pub bytes_written: usize,
}

/// Extracts text from PDF pages concurrently across multiple CPU cores
/// and streams the in-order results to the target file.
pub fn stream_pdf_to_file<P: AsRef<Path>>(source_path: P, dest_file: &mut File) -> Result<ProcessStats, String> {
    let path_ref = source_path.as_ref();

    println!("   ⚙️  Loading PDF document into memory...");
    let mut doc = pdf_extract::Document::load(path_ref)
        .map_err(|e| format!("Failed to load PDF document: {}", e))?;

    if doc.is_encrypted() {
        let _ = doc.decrypt("");
    }

    let pages = doc.get_pages();
    let total_pages = pages.len();
    if total_pages == 0 {
        return Err("No pages found in PDF document".to_string());
    }

    let num_threads = rayon::current_num_threads();
    println!("   ⚡ Dispatching {} pages across {} CPU worker threads...", total_pages, num_threads);

    let page_numbers: Vec<u32> = pages.keys().copied().collect();
    let completed_counter = AtomicUsize::new(0);

    // Parallel extraction across CPU cores
    let extracted_pages: Result<Vec<(u32, String)>, String> = page_numbers
        .par_iter()
        .map(|&page_num| {
            let mut page_text = String::new();
            {
                let mut output = pdf_extract::PlainTextOutput::new(&mut page_text);
                pdf_extract::output_doc_page(&doc, &mut output, page_num)
                    .map_err(|e| format!("Failed to extract page {}: {}", page_num, e))?;
            }

            let done = completed_counter.fetch_add(1, Ordering::Relaxed) + 1;
            println!("   -> [Thread {:?}] Extracted page {}/{} (Progress: {}/{})",
                std::thread::current().id(),
                page_num,
                total_pages,
                done,
                total_pages
            );

            Ok((page_num, page_text))
        })
        .collect();

    let extracted_pages = extracted_pages?;

    println!("   💾 Writing all {} extracted pages to disk in strict sequential order...", total_pages);
    let mut total_bytes = 0;
    for (page_num, page_text) in extracted_pages {
        let bytes = page_text.as_bytes();
        dest_file
            .write_all(bytes)
            .map_err(|e| format!("Failed to write page {} to disk: {}", page_num, e))?;
        total_bytes += bytes.len();
    }

    dest_file.flush().map_err(|e| format!("Flush error: {}", e))?;

    Ok(ProcessStats {
        total_pages,
        bytes_written: total_bytes,
    })
}
