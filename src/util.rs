use std::{fs, path::Path};

pub fn read_file(file_path: &Path) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read file: {}", file_path.display()))
}
