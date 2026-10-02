// src/unloader.rs
use std::fs::File;
use std::io;

/// Safely creates or overwrites the destination file, returning a file handle.
pub fn create_dest_file(dest_path: &str) -> io::Result<File> {
    File::create(dest_path)
}
