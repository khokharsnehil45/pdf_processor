# Streaming PDF Text Extractor

A high-performance, single-threaded command-line interface (CLI) tool built in Rust that extracts text content from PDF files and saves it to a local text file. 

## 🚀 Key Feature: On-The-Fly Streaming
Unlike naive tools that load the entire PDF file and all extracted text into your system's RAM at once, this tool uses a **page-by-page streaming architecture**. 

* **Constant Memory Footprint:** The application processes text one page at a time. Whether your PDF is 5 pages or 5,000 pages, the RAM usage remains incredibly low and steady.
* **On-the-Fly Unloading:** As soon as a page's text is extracted, it is instantly streamed and written directly to your hard drive/SSD before moving to the next page.
* **Real-time Feedback:** Provides instant step-by-step console logging as each page streams.

## 🏗️ Architecture Blueprint
The system is divided into three clean modules to separate concerns:
* `src/main.rs`: Orchestrates the CLI interface, handles user path inputs, and coordinates the processing pipeline.
* `src/loader.rs`: Opens the PDF layout structurally and drives the sequential page extraction loop.
* `src/unloader.rs`: Safely initializes, overwrites, and handles the low-level physical file handles on disk.

## 📋 Prerequisites
Ensure you have the Rust toolchain installed. If not, get it from [rustup.rs](https://rustup.rs).

This project depends on the `pdf-extract` crate. Make sure your `Cargo.toml` contains:
```toml
[dependencies]
pdf-extract = "0.7" # Or your currently targeted version
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
