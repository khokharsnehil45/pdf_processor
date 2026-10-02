use std::io::{self, Write};
use std::time::Instant;

mod loader;
mod unloader;

fn main() {
    // Clear terminal screen and reset cursor
    print!("\x1B[2J\x1B[1;1H");
    
    println!("\x1B[1;36m====================================================\x1B[0m");
    println!("\x1B[1;32m   ⚡ MULTI-CORE STREAMING PDF TEXT EXTRACTOR ⚡   \x1B[0m");
    println!("\x1B[1;36m====================================================\x1B[0m");
    println!(" [Modularity: Enabled]  [Architecture: Multi-Core / Rayon]  [CPU Threads: {}]\n", rayon::current_num_threads());

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

    let pipeline_start = Instant::now();

    // 3. Trigger Unloader to prepare the physical file handle
    match unloader::prepare_destination_file(output_path) {
        Ok((mut file_handle, saved_path)) => {
            println!("  \x1B[32m✔\x1B[0m Unloader Engine: Target file disk handle initialized.");
            println!("  🚀 Extracting pages in parallel across CPU cores and streaming to disk...");

            // 4. Trigger the modular loader loop, passing the file handle by mutable reference
            match loader::stream_pdf_to_file(pdf_path, &mut file_handle) {
                Ok(stats) => {
                    let elapsed = pipeline_start.elapsed();
                    let total_secs = elapsed.as_secs_f64();
                    let avg_ms = if stats.total_pages > 0 {
                        (total_secs * 1000.0) / stats.total_pages as f64
                    } else {
                        0.0
                    };
                    let formatted_size = if stats.bytes_written < 1024 {
                        format!("{} B", stats.bytes_written)
                    } else if stats.bytes_written < 1024 * 1024 {
                        format!("{:.2} KB", stats.bytes_written as f64 / 1024.0)
                    } else {
                        format!("{:.2} MB", stats.bytes_written as f64 / (1024.0 * 1024.0))
                    };

                    println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
                    println!("\n\x1B[1;92m🎉 SUCCESS:\x1B[0m Entire PDF processed and saved to \x1B[4;32m'{}'\x1B[0m", saved_path);
                    println!("  ⏱️  \x1B[1;33mTime Elapsed:\x1B[0m       {:.3} s ({:.1} ms/page)", total_secs, avg_ms);
                    println!("  📊 \x1B[1;36mTotal Pages:\x1B[0m        {}", stats.total_pages);
                    println!("  💾 \x1B[1;35mData Written:\x1B[0m       {}\n", formatted_size);
                }
                Err(stream_error) => {
                    let elapsed = pipeline_start.elapsed();
                    println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
                    eprintln!("\n\x1B[1;31m❌ PIPELINE CRASH during Streaming (after {:.2}s):\x1B[0m\n   {}", elapsed.as_secs_f64(), stream_error);
                }
            }
        }
        Err(prepare_error) => {
            let elapsed = pipeline_start.elapsed();
            println!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
            eprintln!("\n\x1B[1;31m❌ PIPELINE CRASH at Unloader Initialization (after {:.2}s):\x1B[0m\n   {}", elapsed.as_secs_f64(), prepare_error);
        }
    }
}
