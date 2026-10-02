# Multi-Core PDF Text Extractor

A high-performance command-line interface (CLI) tool built in Rust that extracts text content from PDF files across multiple CPU cores in parallel and saves the ordered text to a local file.

## 🚀 Key Features

* **Multi-Core Parallel Extraction:** Uses `rayon` to extract pages simultaneously across all available CPU threads while preserving strict sequential page order in the output.
* **Low-Latency Streaming:** Streams extracted text straight to disk without keeping unnecessary file buffers.
* **Real-time Pipeline Metrics:** Displays live per-thread extraction progress, total processing time, speed per page (ms/page), and data written.

## 🏗️ Architecture Blueprint
The system is divided into clean modules:
* `src/main.rs`: Orchestrates the interactive CLI interface, captures execution metrics, and coordinates the processing pipeline.
* `src/loader.rs`: Loads the PDF structure, dispatches pages across the `rayon` thread pool, and collects in-order output.
* `src/unloader.rs`: Safely initializes, creates directories if needed, and handles the physical file handle on disk.

## 📋 Prerequisites
Ensure you have the Rust toolchain installed from [rustup.rs](https://rustup.rs).

Dependencies in `Cargo.toml`:
```toml
[dependencies]
pdf-extract = "0.7.0"
rayon = "1.12.0"
```

## 🛠️ How to Run

1. Open your terminal in the project directory.
2. Run the application using Cargo:
   ```bash
   cargo run
   ```
3. Follow the interactive CLI prompts:
   ```text
   === Streaming PDF Text Extractor ===
   Enter the path to the source PDF file: documents/sample.pdf
   Enter destination file path (Press Enter for default: 'output.txt'): exports/result.txt

   Preparing destination file...
   Streaming text page-by-page from PDF to disk...
      -> Streamed page 1 successfully
      -> Streamed page 2 successfully
      -> Streamed page 3 successfully

   🎉 Success! Entire PDF streamed and saved to 'exports/result.txt'.
   ```

## 📄 License
This project is open-source and available under the MIT License.
