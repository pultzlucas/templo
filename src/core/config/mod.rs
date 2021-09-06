use super::path::{get_config_path, get_repo_path};
use std::path::Path;
use crate::utils::errors::std_error;
use crate::utils::path::pathbuf_to_string;
use serde_derive::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fs;
use std::io::Error;

pub mod repos;

#[derive(Serialize, Deserialize)]
pub struct RemoteRepoRegistry {
    pub name: String,
    pub url: String,
    pub authorization: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LocalRepoRegistry {
    pub name: String,
    pub path: String,
}

pub fn exists() -> bool {
    Path::new(&get_config_path().unwrap()).exists()
}

pub fn create_files() -> Result<(), Error> {
    if exists() {
        return Ok(());
    } 
    
    let config_dir = get_config_path()?;
    fs::create_dir(&config_dir)?;

    // create repos config files
    let main_local_repo = LocalRepoRegistry {
        name: "main".to_string(),
        path: pathbuf_to_string(get_repo_path()?),
    };

    let std_tools_repo = RemoteRepoRegistry {
        name: "std-tools".to_string(),
        url: "https://templo-std-tools.herokuapp.com/v1/templates/{}".to_string(),
        authorization: None,
    };

    let repos_dir = config_dir.join("Repos");

    fs::create_dir(&repos_dir)?;

    fs::write(
        &repos_dir.join("local.json"),
        std_error(to_string_pretty(&vec![main_local_repo]))?,
    )?;

    fs::write(
        repos_dir.join("remote.json"), 
        std_error(to_string_pretty(&vec![std_tools_repo]))?
    )?;

    Ok(())
}
