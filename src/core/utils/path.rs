use super::errors::invalid_input_error;
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn pathbuf_to_string(path: PathBuf) -> String {
    path.as_os_str().to_str().unwrap().to_string()
}

pub fn format_pathbuf(path: PathBuf) -> PathBuf {
    Path::new(&pathbuf_to_string(path).replace(r"\", "/")).to_path_buf()
}

pub fn valid_directory_path(directory: &str) -> Result<(), Error> {
    if directory.contains(r"\") || directory.ends_with("/") {
        return Err(invalid_input_error("Invalid directory path."));
    }
    Ok(())
}
