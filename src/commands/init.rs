use crate::core::{
    file_system::paths::TEMPLATES_PATH,
    repository::template_repository_exists
};
use std::{fs, io::Error};

pub fn init() -> Result<(), Error> {
    if !template_repository_exists() {
        fs::create_dir_all(TEMPLATES_PATH)?;
        println!("Repository was created.");
    }

    Ok(())
}
