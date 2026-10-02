use std::fs::{self, File};
use std::path::Path;

/// Safely prepares and creates the target file handle on disk.
pub fn prepare_destination_file(output_path: &str) -> Result<(File, String), String> {
    let final_path_str = if output_path.trim().is_empty() {
        "output.txt"
    } else {
        output_path
    };

    let path = Path::new(final_path_str);

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            if let Err(err) = fs::create_dir_all(parent) {
                return Err(format!("Failed to create directories: {}", err));
            }
        }
    }

    match File::create(path) {
        Ok(file_handle) => Ok((file_handle, final_path_str.to_string())),
        Err(err) => Err(format!("Failed to create file '{}': {}", final_path_str, err)),
    }
}
