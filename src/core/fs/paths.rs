use crate::core::info::os_is_windows;
use crate::utils::errors::std_error;
use std::env::var;
use std::io::Error;
use std::path::PathBuf;

pub fn get_templates_path() -> Result<PathBuf, Error> {
    Ok(get_repo_path()?.join("Templates"))
}

pub fn get_repo_path() -> Result<PathBuf, Error> {
    let data_path = get_app_data_path()?;
    Ok(data_path.join("Repository"))
}

pub fn get_user_auth_path() -> Result<PathBuf, Error> {
    Ok(get_app_data_path()?.join("auth"))
}

fn get_app_data_path() -> Result<PathBuf, Error> {
    if os_is_windows() {
        let home = std_error(var("XDG_CONFIG_HOME").or_else(|_| var("HOME")))?;
        let data_path = PathBuf::from(home)
            .join("AppData")
            .join("Local")
            .join("Prottern");

        return Ok(data_path);
    }

    panic!("Invalid OS target.")
}
