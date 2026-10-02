use std::io::{self, Write};
use std::time::Instant;

mod loader;
mod structurer;
mod unloader;

fn main() {
    // Clear terminal screen and reset cursor
    print!("\x1B[2J\x1B[1;1H");
    
    println!("\x1B[1;36m====================================================\x1B[0m");
    println!("\x1B[1;32m   ⚡ MULTI-CORE AI / JSON PDF STRUCTURER ⚡        \x1B[0m");
    println!("\x1B[1;36m====================================================\x1B[0m");
    println!(" [Modularity: Enabled]  [Architecture: Multi-Core / Rayon]  [CPU Threads: {}]\n", rayon::current_num_threads());

    // 1. Get Source PDF File Path
    print!("\x1B[1;33m👉 Step 1: Enter source PDF path:\x1B[0m\n   ↳ ");
    io::stdout().flush().unwrap();
    let mut pdf_path = String::new();
    io::stdin().read_line(&mut pdf_path).expect("Failed to read line");
    let pdf_path = pdf_path.trim();

    // 2. Get Destination Target Path
    print!("\n\x1B[1;33m👉 Step 2: Enter destination path (Press Enter for 'output.json'):\x1B[0m\n   ↳ ");
    io::stdout().flush().unwrap();
    let mut output_path = String::new();
    io::stdin().read_line(&mut output_path).expect("Failed to read line");
    let output_path = output_path.trim();

    println!("\n\x1B[1;35m⚙️  Executing Pipeline Blocks...\x1B[0m");
    println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");

    let pipeline_start = Instant::now();

    // Block 1: Loader Engine (Multi-core parallel extraction)
    println!("  🚀 \x1B[1mStage 1/3 [Loader Engine]:\x1B[0m Extracting pages in parallel across CPU cores...");
    let extracted_pages = match loader::extract_pages_parallel(pdf_path) {
        Ok(pages) => pages,
        Err(err) => {
            let elapsed = pipeline_start.elapsed();
            println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
            eprintln!("\n\x1B[1;31m❌ PIPELINE CRASH during Page Extraction (after {:.2}s):\x1B[0m\n   {}", elapsed.as_secs_f64(), err);
            return;
        }
    };

    let total_pages = extracted_pages.len();

    // Block 2: Structurer Engine (AI / Vector DB JSON model generation)
    println!("  🧠 \x1B[1mStage 2/3 [Structurer Engine]:\x1B[0m Building structured JSON payload with metadata...");
    let structured_doc = structurer::StructuredDocument::from_extracted_pages(pdf_path, extracted_pages);

    // Block 3: Unloader Engine (Writing formatted output to disk)
    println!("  💾 \x1B[1mStage 3/3 [Unloader Engine]:\x1B[0m Serializing and writing to target file...");
    match unloader::save_structured_document(output_path, &structured_doc) {
        Ok(save_result) => {
            let elapsed = pipeline_start.elapsed();
            let total_secs = elapsed.as_secs_f64();
            let avg_ms = if total_pages > 0 {
                (total_secs * 1000.0) / total_pages as f64
            } else {
                0.0
            };
            let formatted_size = if save_result.bytes_written < 1024 {
                format!("{} B", save_result.bytes_written)
            } else if save_result.bytes_written < 1024 * 1024 {
                format!("{:.2} KB", save_result.bytes_written as f64 / 1024.0)
            } else {
                format!("{:.2} MB", save_result.bytes_written as f64 / (1024.0 * 1024.0))
            };

            println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
            println!("\n\x1B[1;92m🎉 SUCCESS:\x1B[0m Structured document generated and saved to \x1B[4;32m'{}'\x1B[0m", save_result.saved_path);
            println!("  📋 \x1B[1;34mFormat:\x1B[0m             {}", save_result.format.description());
            println!("  ⏱️  \x1B[1;33mTime Elapsed:\x1B[0m       {:.3} s ({:.1} ms/page)", total_secs, avg_ms);
            println!("  📊 \x1B[1;36mTotal Pages:\x1B[0m        {}", structured_doc.metadata.total_pages);
            println!("  🔤 \x1B[1;37mTotal Characters:\x1B[0m   {}", structured_doc.metadata.total_characters);
            println!("  📝 \x1B[1;32mTotal Words:\x1B[0m        {}", structured_doc.metadata.total_words);
            println!("  💾 \x1B[1;35mData Written:\x1B[0m       {}\n", formatted_size);
        }
        Err(err) => {
            let elapsed = pipeline_start.elapsed();
            println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
            eprintln!("\n\x1B[1;31m❌ PIPELINE CRASH at Unloader Serialization (after {:.2}s):\x1B[0m\n   {}", elapsed.as_secs_f64(), err);
        }
    }
}
