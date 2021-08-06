use crate::core::file_system::paths::TEMPLATES_PATH;
use std::path::Path;
use std::io::Error;
use std::fs;

mod repository_connection;
mod serde;
pub use repository_connection::*;

pub fn repository_exists() -> bool {
    Path::new(TEMPLATES_PATH).exists()
}

pub fn create_repository_if_not_exists() -> Result<(), Error> {
    if !repository_exists() {
        fs::create_dir_all(TEMPLATES_PATH)?;
        println!("Repository was created.");
    }

    Ok(())
}
