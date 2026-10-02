# Multi-Core PDF Text Extractor & AI Structurer

A high-performance command-line interface (CLI) tool built in Rust that extracts text content from PDF files across multiple CPU cores in parallel and structures it into clean, AI/LLM & Vector DB ingestible formats (`JSON`, `JSONL`, or `TXT`).

## 🚀 Key Features

* **Multi-Core Parallel Extraction:** Uses `rayon` to extract pages simultaneously across all available CPU cores while preserving strict sequential page order.
* **AI / Vector DB Structurer:** Transforms raw text into structured JSON payloads containing `page_number`, `character_count`, `word_count`, `text_payload`, and document-level metadata (`source_file`, `total_pages`, `total_characters`, `total_words`, `extraction_timestamp`).
* **Multi-Format Export:**
  - **Structured JSON (`.json`)**: Formatted pretty JSON document envelope with metadata and page arrays.
  - **JSON Lines (`.jsonl`)**: Line-delimited JSON objects for direct batch ingestion into vector stores (Pinecone, Qdrant, Chroma, Milvus).
  - **Plain Text (`.txt`)**: Raw sequential plain text export.
* **Real-time Pipeline Metrics:** Displays live per-thread extraction progress, total processing time, speed per page (ms/page), and data written.

## 🏗️ Architecture Blueprint
The system is divided into modular pipeline blocks:
* `src/main.rs`: Orchestrates user input, pipeline stage execution, performance timing, and terminal telemetry.
* `src/loader.rs`: Loads PDF document structures and orchestrates parallel multi-core page extraction via `rayon`.
* `src/structurer.rs`: AI structurer engine that builds `StructuredDocument` and `PagePayload` objects with statistical metrics.
* `src/unloader.rs`: File system unloader that creates destination paths and serializes to JSON, JSONL, or plain text.

## 📋 Prerequisites
Ensure you have the Rust toolchain installed from [rustup.rs](https://rustup.rs).

Dependencies in `Cargo.toml`:
```toml
[dependencies]
pdf-extract = "0.7.0"
rayon = "1.12.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

## 📦 JSON Payload Schema

```json
{
  "metadata": {
    "source_file": "document.pdf",
    "total_pages": 24,
    "total_characters": 94805,
    "total_words": 15385,
    "extraction_timestamp": "2026-10-02T10:18:49Z"
  },
  "pages": [
    {
      "page_number": 1,
      "character_count": 4400,
      "word_count": 681,
      "text_payload": "..."
    }
  ]
}
```

## 🛠️ How to Run

### Option 1: Direct Pipe into AI LLMs (e.g. Ollama)
```bash
# Pipe structured JSON directly into Ollama
PDFL -load document.pdf -json | ollama run llama3.2:3b "Summarize this document:"

# Pipe with quiet mode (clean stdout only)
PDFL -load document.pdf -json -q | jq .metadata
```

### Option 2: Global CLI Command (Interactive Mode)
```bash
PDFL
```
*(or `pdfl`)* from any terminal/directory in the system.

### Option 3: Command Line Flags
```bash
PDFL -load <file_path> -json              # Stream structured JSON to stdout
PDFL -load <file_path> -jsonl             # Stream JSON Lines to stdout
PDFL -load <file_path> -text              # Stream plain text to stdout
PDFL -load <file_path> -o <output_file>   # Save to .json, .jsonl, or .txt
PDFL -load <file_path> -json -q           # Quiet mode (silences stderr logs)
```

## 📄 License
This project is open-source and available under the MIT License.
