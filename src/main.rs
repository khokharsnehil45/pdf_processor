// src/main.rs
use std::io::{self, Write};
use std::path::PathBuf;

// Register our loader and unloader modules
mod loader;
mod unloader;

fn main() {
    println!("=== PDF Text Extractor CLI ===");

    // 1. Get the source PDF path
    let source_path = prompt_user("Enter the path to the source PDF file: ");
    if source_path.is_empty() {
        println!("Error: Source path cannot be empty.");
        return;
    }

    // 2. Get the destination file path (with a default option)
    let default_dest = "output.txt";
    let mut dest_input = prompt_user(&format!(
        "Enter destination file path (Press Enter for default: '{}'): ", 
        default_dest
    ));

    if dest_input.is_empty() {
        dest_input = default_dest.to_string();
    }

    println!("\n[1/2] Loading and extracting PDF contents...");
    
    // 3. Orchestrate the flow using our components
    match loader::load_pdf(&source_path) {
        Ok(extracted_text) => {
            println!("[2/2] Saving extracted text to '{}'...", dest_input);
            
            match unloader::unload_text(&extracted_text, &dest_input) {
                Ok(_) => println!("\n🎉 Success! Content successfully processed and saved."),
                Err(e) => println!("\n❌ Unloader Error: {}", e),
            }
        }
        Err(e) => println!("\n❌ Loader Error: {}", e),
    }
}

/// Helper function to cleanly read user input from the terminal
fn prompt_user(msg: &str) -> String {
    print!("{}", msg);
    let _ = io::stdout().flush(); // Ensure the prompt prints immediately

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
