// src/main.rs
use std::io::{self, Write};

mod loader;
mod unloader;

fn main() {
    println!("=== Streaming PDF Text Extractor ===");

    // 1. Get Source Path
    print!("Enter the path to the source PDF file: ");
    io::stdout().flush().unwrap();
    let mut source_path = String::new();
    io::stdin().read_line(&mut source_path).unwrap();
    let source_path = source_path.trim();

    // 2. Get Destination Path
    print!("Enter destination file path (Press Enter for default: 'output.txt'): ");
    io::stdout().flush().unwrap();
    let mut dest_path = String::new();
    io::stdin().read_line(&mut dest_path).unwrap();
    let mut dest_path = dest_path.trim().to_string();
    if dest_path.is_empty() {
        dest_path = String::from("output.txt");
    }

    // 3. Streaming Pipeline
    println!("\nPreparing destination file...");
    match unloader::create_dest_file(&dest_path) {
        Ok(mut dest_file) => {
            println!("Streaming text page-by-page from PDF to disk...");
            match loader::stream_pdf_to_file(source_path, &mut dest_file) {
                Ok(_) => println!("\n🎉 Success! Entire PDF streamed and saved to '{}'.", dest_path),
                Err(e) => eprintln!("\n❌ Extraction error: {}", e),
            }
        }
        Err(e) => {
            eprintln!("❌ Error creating destination file: {}", e);
        }
    }
}
