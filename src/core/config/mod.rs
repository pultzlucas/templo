use crate::core::path::{get_config_path, get_repo_path};
use std::path::Path;
use crate::utils::errors::std_error;
use crate::utils::path::pathbuf_to_string;
use serde_derive::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fs;
use std::io::Error;
use crate::paintln;
use tabled::Tabled;

pub mod repos;

#[derive(Serialize, Deserialize, Tabled, Debug, Clone)]
pub struct RemoteRepoRegistry {
    pub name: String,
    pub url: String,
    pub requires_authorization: bool,
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

    paintln!("{gray}", "[creating config files]");
    let config_dir = get_config_path()?;
    fs::create_dir(&config_dir)?;

    // create repos config files
    let main_local_repo = LocalRepoRegistry {
        name: "main".to_string(),
        path: pathbuf_to_string(get_repo_path()?),
    };

    let std_tools_repo = RemoteRepoRegistry {
        name: "std-tools".to_string(),
        url: "https://templo-std-tools.herokuapp.com/v1".to_string(),
        requires_authorization: false,
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

    println!("Config files was created.");

    Ok(())
}
