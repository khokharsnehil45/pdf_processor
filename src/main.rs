use std::io::{self, Write};

mod loader;
mod unloader;

fn main() {
    // Clear terminal screen and reset cursor
    print!("\x1B[2J\x1B[1;1H");
    
    println!("\x1B[1;36m====================================================\x1B[0m");
    println!("\x1B[1;32m   ⚡ LOW-LATENCY STREAMING PDF TEXT EXTRACTOR ⚡   \x1B[0m");
    println!("\x1B[1;36m====================================================\x1B[0m");
    println!(" [Modularity: Enabled]  [Architecture: Streaming] \n");

    // 1. Get Source PDF File Path
    print!("\x1B[1;33m👉 Step 1: Enter source PDF path:\x1B[0m\n   ↳ ");
    io::stdout().flush().unwrap();
    let mut pdf_path = String::new();
    io::stdin().read_line(&mut pdf_path).expect("Failed to read line");
    let pdf_path = pdf_path.trim();

    // 2. Get Destination Target Path
    print!("\n\x1B[1;33m👉 Step 2: Enter destination path (Press Enter for 'output.txt'):\x1B[0m\n   ↳ ");
    io::stdout().flush().unwrap();
    let mut output_path = String::new();
    io::stdin().read_line(&mut output_path).expect("Failed to read line");
    let output_path = output_path.trim();

    println!("\n\x1B[1;35m⚙️  Executing Pipeline Blocks...\x1B[0m");
    println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");

    // 3. Trigger Unloader to prepare the physical file handle
    match unloader::prepare_destination_file(output_path) {
        Ok((mut file_handle, saved_path)) => {
            println!("  \x1B[32m✔\x1B[0m Unloader Engine: Target file disk handle initialized.");
            println!("  🚀 Streaming text page-by-page from PDF directly to disk...");

            // 4. Trigger the new modular loader loop, passing the file handle by mutable reference
            match loader::stream_pdf_to_file(pdf_path, &mut file_handle) {
                Ok(_) => {
                    println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
                    println!("\n\x1B[1;92m🎉 SUCCESS:\x1B[0m Entire PDF streamed and saved to \x1B[4;32m'{}'\x1B[0m\n", saved_path);
                }
                Err(stream_error) => {
                    println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
                    eprintln!("\n\x1B[1;31m❌ PIPELINE CRASH during Streaming:\x1B[0m\n   {}", stream_error);
                }
            }
        }
        Err(prepare_error) => {
            println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
            eprintln!("\n\x1B[1;31m❌ PIPELINE CRASH at Unloader Initialization:\x1B[0m\n   {}", prepare_error);
        }
    }
}
