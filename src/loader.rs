use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;

/// Extracts text from PDF pages concurrently across multiple CPU cores
/// and returns the ordered results `Vec<(page_number, text)>`.
/// Diagnostic/progress messages are written to stderr so stdout remains clean for piping.
pub fn extract_pages_parallel<P: AsRef<Path>>(source_path: P, quiet: bool) -> Result<Vec<(u32, String)>, String> {
    let path_ref = source_path.as_ref();

    if !quiet {
        eprintln!("   ⚙️  Loading PDF document into memory...");
    }

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
    if !quiet {
        eprintln!("   ⚡ Dispatching {} pages across {} CPU worker threads...", total_pages, num_threads);
    }

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
            if !quiet {
                eprintln!("   -> [Thread {:?}] Extracted page {}/{} (Progress: {}/{})",
                    std::thread::current().id(),
                    page_num,
                    total_pages,
                    done,
                    total_pages
                );
            }

            Ok((page_num, page_text))
        })
        .collect();

    extracted_pages
}
