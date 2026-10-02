use std::io::{self, Write};
use std::time::Instant;

mod loader;
mod structurer;
mod unloader;

#[derive(Debug, Clone, PartialEq, Eq)]
enum OutputMode {
    StdoutJson,
    StdoutJsonLines,
    StdoutText,
    File(String),
}

struct CliConfig {
    pdf_path: Option<String>,
    output_mode: OutputMode,
    quiet: bool,
    interactive: bool,
}

fn print_help() {
    eprintln!("\x1B[1;36m====================================================\x1B[0m");
    eprintln!("\x1B[1;32m   ⚡ MULTI-CORE AI / JSON PDF STRUCTURER (PDFL) ⚡  \x1B[0m");
    eprintln!("\x1B[1;36m====================================================\x1B[0m\n");
    eprintln!("\x1B[1mUSAGE:\x1B[0m");
    eprintln!("  PDFL -load <file_path> -json [OPTIONS]              # Pipe structured JSON to stdout");
    eprintln!("  PDFL -load <file_path> -json | ollama run <model>   # Direct pipe into local LLMs");
    eprintln!("  PDFL -load <file_path> -jsonl                       # Output JSON Lines (Vector DB batch)");
    eprintln!("  PDFL -load <file_path> -o <destination_path>        # Save to file (.json, .jsonl, .txt)");
    eprintln!("  PDFL                                                # Interactive CLI wizard\n");
    eprintln!("\x1B[1mOPTIONS:\x1B[0m");
    eprintln!("  -load, --load <path>    Path to the source PDF document");
    eprintln!("  -json, --json           Stream structured JSON payload to standard output");
    eprintln!("  -jsonl, --jsonl         Stream line-delimited JSON (JSONL) to standard output");
    eprintln!("  -text, --text, -raw     Stream raw plain text to standard output");
    eprintln!("  -o, --output <path>     Save output directly to specified file");
    eprintln!("  -q, --quiet             Suppress all diagnostic/progress logs on stderr");
    eprintln!("  -h, --help              Print this help documentation");
}

fn parse_args() -> Result<CliConfig, String> {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();

    if raw_args.is_empty() {
        return Ok(CliConfig {
            pdf_path: None,
            output_mode: OutputMode::File("output.json".to_string()),
            quiet: false,
            interactive: true,
        });
    }

    let mut pdf_path: Option<String> = None;
    let mut output_mode = OutputMode::StdoutJson;
    let mut quiet = false;

    let mut i = 0;
    while i < raw_args.len() {
        match raw_args[i].as_str() {
            "-load" | "--load" | "-l" => {
                i += 1;
                if i >= raw_args.len() {
                    return Err("Flag '-load' requires a valid PDF file path.".to_string());
                }
                pdf_path = Some(raw_args[i].clone());
            }
            "-json" | "--json" => {
                output_mode = OutputMode::StdoutJson;
            }
            "-jsonl" | "--jsonl" => {
                output_mode = OutputMode::StdoutJsonLines;
            }
            "-text" | "--text" | "-raw" | "--raw" => {
                output_mode = OutputMode::StdoutText;
            }
            "-o" | "--output" | "-out" | "--out" => {
                i += 1;
                if i >= raw_args.len() {
                    return Err("Flag '-o' requires an output file path.".to_string());
                }
                output_mode = OutputMode::File(raw_args[i].clone());
            }
            "-q" | "--quiet" => {
                quiet = true;
            }
            "-h" | "--help" | "help" => {
                print_help();
                std::process::exit(0);
            }
            arg if !arg.starts_with('-') && pdf_path.is_none() => {
                pdf_path = Some(arg.to_string());
            }
            unknown => {
                return Err(format!("Unknown option '{}'. Run 'PDFL --help' for usage.", unknown));
            }
        }
        i += 1;
    }

    Ok(CliConfig {
        pdf_path,
        output_mode,
        quiet,
        interactive: false,
    })
}

fn main() {
    let config = match parse_args() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("\x1B[1;31mError:\x1B[0m {}", err);
            std::process::exit(1);
        }
    };

    let (pdf_path, output_mode, quiet) = if config.interactive {
        // Clear terminal screen and reset cursor for interactive wizard
        print!("\x1B[2J\x1B[1;1H");
        
        eprintln!("\x1B[1;36m====================================================\x1B[0m");
        eprintln!("\x1B[1;32m   ⚡ MULTI-CORE AI / JSON PDF STRUCTURER ⚡        \x1B[0m");
        eprintln!("\x1B[1;36m====================================================\x1B[0m");
        eprintln!(" [Modularity: Enabled]  [Architecture: Multi-Core / Rayon]  [CPU Threads: {}]\n", rayon::current_num_threads());

        // 1. Get Source PDF File Path
        eprint!("\x1B[1;33m👉 Step 1: Enter source PDF path:\x1B[0m\n   ↳ ");
        io::stderr().flush().unwrap();
        let mut path_in = String::new();
        io::stdin().read_line(&mut path_in).expect("Failed to read line");
        let path_in = path_in.trim().to_string();

        if path_in.is_empty() {
            eprintln!("\x1B[1;31mError:\x1B[0m PDF path cannot be empty.");
            std::process::exit(1);
        }

        // 2. Get Destination Target Path
        eprint!("\n\x1B[1;33m👉 Step 2: Enter destination path (Press Enter for 'output.json'):\x1B[0m\n   ↳ ");
        io::stderr().flush().unwrap();
        let mut dest_in = String::new();
        io::stdin().read_line(&mut dest_in).expect("Failed to read line");
        let dest_in = dest_in.trim().to_string();
        let final_dest = if dest_in.is_empty() { "output.json".to_string() } else { dest_in };

        eprintln!("\n\x1B[1;35m⚙️  Executing Pipeline Blocks...\x1B[0m");
        eprintln!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");

        (path_in, OutputMode::File(final_dest), false)
    } else {
        let path = match config.pdf_path {
            Some(p) if !p.trim().is_empty() => p.trim().to_string(),
            _ => {
                eprintln!("\x1B[1;31mError:\x1B[0m Missing PDF file path. Specify via '-load <file_path>' or run without arguments for interactive mode.");
                std::process::exit(1);
            }
        };
        (path, config.output_mode, config.quiet)
    };

    let pipeline_start = Instant::now();

    // Stage 1: Parallel multi-core page extraction
    if !quiet {
        eprintln!("  🚀 \x1B[1mStage 1/3 [Loader Engine]:\x1B[0m Extracting pages in parallel across CPU cores...");
    }

    let extracted_pages = match loader::extract_pages_parallel(&pdf_path, quiet) {
        Ok(pages) => pages,
        Err(err) => {
            let elapsed = pipeline_start.elapsed();
            if !quiet {
                eprintln!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
            }
            eprintln!("\n\x1B[1;31m❌ PIPELINE CRASH during Page Extraction (after {:.2}s):\x1B[0m\n   {}", elapsed.as_secs_f64(), err);
            std::process::exit(1);
        }
    };

    let total_pages = extracted_pages.len();

    // Stage 2: Structurer Engine (AI / Vector DB JSON model generation)
    if !quiet {
        eprintln!("  🧠 \x1B[1mStage 2/3 [Structurer Engine]:\x1B[0m Building structured JSON payload with metadata...");
    }
    let structured_doc = structurer::StructuredDocument::from_extracted_pages(&pdf_path, extracted_pages);

    // Stage 3: Unloader / Output Delivery
    match output_mode {
        OutputMode::StdoutJson => {
            let json_str = match structured_doc.to_json_pretty() {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("JSON serialization error: {}", err);
                    std::process::exit(1);
                }
            };
            println!("{}", json_str);
            let _ = io::stdout().flush();

            if !quiet {
                let elapsed = pipeline_start.elapsed();
                eprintln!("\n\x1B[1;92m✔ Pipeline Complete:\x1B[0m {} pages extracted in {:.3}s. Structured JSON emitted to stdout.", total_pages, elapsed.as_secs_f64());
            }
        }
        OutputMode::StdoutJsonLines => {
            let jsonl_str = match structured_doc.to_jsonl() {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("JSONL serialization error: {}", err);
                    std::process::exit(1);
                }
            };
            print!("{}", jsonl_str);
            let _ = io::stdout().flush();

            if !quiet {
                let elapsed = pipeline_start.elapsed();
                eprintln!("\n\x1B[1;92m✔ Pipeline Complete:\x1B[0m {} pages extracted in {:.3}s. JSONL emitted to stdout.", total_pages, elapsed.as_secs_f64());
            }
        }
        OutputMode::StdoutText => {
            for page in &structured_doc.pages {
                print!("{}", page.text_payload);
            }
            let _ = io::stdout().flush();

            if !quiet {
                let elapsed = pipeline_start.elapsed();
                eprintln!("\n\x1B[1;92m✔ Pipeline Complete:\x1B[0m {} pages extracted in {:.3}s. Plain text emitted to stdout.", total_pages, elapsed.as_secs_f64());
            }
        }
        OutputMode::File(ref dest_path) => {
            if !quiet {
                eprintln!("  💾 \x1B[1mStage 3/3 [Unloader Engine]:\x1B[0m Serializing and writing to target file...");
            }
            match unloader::save_structured_document(dest_path, &structured_doc) {
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

                    if !quiet {
                        eprintln!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
                        eprintln!("\n\x1B[1;92m🎉 SUCCESS:\x1B[0m Structured document generated and saved to \x1B[4;32m'{}'\x1B[0m", save_result.saved_path);
                        eprintln!("  📋 \x1B[1;34mFormat:\x1B[0m             {}", save_result.format.description());
                        eprintln!("  ⏱️  \x1B[1;33mTime Elapsed:\x1B[0m       {:.3} s ({:.1} ms/page)", total_secs, avg_ms);
                        eprintln!("  📊 \x1B[1;36mTotal Pages:\x1B[0m        {}", structured_doc.metadata.total_pages);
                        eprintln!("  🔤 \x1B[1;37mTotal Characters:\x1B[0m   {}", structured_doc.metadata.total_characters);
                        eprintln!("  📝 \x1B[1;32mTotal Words:\x1B[0m        {}", structured_doc.metadata.total_words);
                        eprintln!("  💾 \x1B[1;35mData Written:\x1B[0m       {}\n", formatted_size);
                    }
                }
                Err(err) => {
                    let elapsed = pipeline_start.elapsed();
                    if !quiet {
                        eprintln!("\x1B[36m────────────────────────────────────────────────────\x1B[0m");
                    }
                    eprintln!("\n\x1B[1;31m❌ PIPELINE CRASH at Unloader Serialization (after {:.2}s):\x1B[0m\n   {}", elapsed.as_secs_f64(), err);
                    std::process::exit(1);
                }
            }
        }
    }
}
