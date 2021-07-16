use crate::core::{
    file_system::paths::TEMPLATES_PATH,
    repository::RepositoryConnection
};
use std::{fs, io::Error};

pub fn init() -> Result<(), Error> {
    if !RepositoryConnection::repository_exists() {
        fs::create_dir_all(TEMPLATES_PATH)?;
        println!("Repository was created.");
    }

    Ok(())
}
