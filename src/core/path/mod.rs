use crate::core::info::os_is_windows;
use crate::utils::errors::not_found_error;
use std::io::Error;
use std::path::PathBuf;

pub fn get_config_path() -> Result<PathBuf, Error> {
    Ok(get_app_local_path()?.join("Config"))
}

pub fn get_templates_path() -> Result<PathBuf, Error> {
    Ok(get_repo_path()?.join("Templates"))
}

pub fn get_repo_path() -> Result<PathBuf, Error> {
    let data_path = get_app_local_path()?;
    Ok(data_path.join("Repository"))
}

pub fn get_app_local_path() -> Result<PathBuf, Error> {
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => return Err(not_found_error("Not is possible to get your home folder."))
    };

    if os_is_windows() {
        let data_path = home_dir
            .join("AppData")
            .join("Local")
            .join("Templo");

        return Ok(data_path);
    }

    panic!("Invalid OS target.")
}
