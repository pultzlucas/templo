use crate::core::{
    file_system::paths::TEMPLATES_PATH,
    repository::repository_exists
};
use std::{fs, io::Error};

pub fn init() -> Result<(), Error> {
    if !repository_exists() {
        fs::create_dir_all(TEMPLATES_PATH)?;
        println!("Repository was created.");
    }

    Ok(())
}
