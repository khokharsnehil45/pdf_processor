# PDF Text Extractor CLI

A lightweight, modular **command-line interface (CLI) tool** built in **Rust** that extracts raw text content from PDF documents and saves it directly to a local file. 

This project demonstrates a clean, component-based architecture separation between file I/O operations and user interaction loops.

---

## 🚀 Features

* 🖥️ **Interactive CLI:** A simple terminal interface that prompts users for input and destination paths.
* 📦 **Modular Architecture:** Split into isolated components for robust maintenance:
  * `loader.rs` handles reading the binary bytes and safely extracting text.
  * `unloader.rs` manages file creation, automated directory nesting, and disk writes.
  * `main.rs` orchestrates the CLI application flow.
* 💾 **Smart Defaults:** Instantly falls back to a default `output.txt` if the user leaves the destination prompt blank.
* 📁 **Auto-Folder Creation:** Automatically generates nested directory paths if the specified destination directory does not exist yet.

---

## 🛠️ Project Structure

```text
pdf_processor/
├── Cargo.toml
└── src/
    ├── main.rs        # CLI UI and Execution Coordinator
    ├── loader.rs      # PDF Ingestion Component
    └── unloader.rs    # File Export Component
```

---

## 📋 Prerequisites

To build and run this project, make sure you have the Rust toolchain installed:

```bash
# Verify Rust installation
cargo --version
```

---

## ⚙️ Installation & Setup

1. **Clone or locate your project directory:**
   ```bash
   cd pdf_processor
   ```

2. **Verify your `Cargo.toml` dependencies:**
   Ensure your `Cargo.toml` file includes the `pdf-extract` crate:
   ```toml
   [dependencies]
   pdf-extract = "0.7.0"
   ```

3. **Build the project:**
   ```bash
   cargo build --release
   ```

---

## 📖 Usage

Run the tool using `cargo`:

```bash
cargo run
```

### Example Walkthrough

```text
=== PDF Text Extractor CLI ===
Enter the path to the source PDF file: /path/to/my_document.pdf
Enter destination file path (Press Enter for default: 'output.txt'): exports/result.txt

[1/2] Loading and extracting PDF contents...
[2/2] Saving extracted text to 'exports/result.txt'...

🎉 Success! Content successfully processed and saved.
```

---

## 🧩 Components Deep Dive

### 1. Loader (`src/loader.rs`)
Reads the file paths provided by the orchestrator, loads the raw binary structure into an in-memory buffer, and uses the `pdf-extract` engine to synthesize string sequences.

### 2. Unloader (`src/unloader.rs`)
Safeguards the data saving process. It queries the operating system for directory structures, provisions lacking folders on demand, and cleanly overwrites or produces the resulting plain-text file.
