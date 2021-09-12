use crate::core::path::get_config_path;
use crate::paintln;
use crate::utils::errors::std_error;
use serde_json::to_string_pretty;
use std::fs;
use std::io::Error;
use std::path::Path;

pub mod registry;

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

    let std_tools_repo = registry::RemoteRepoRegistry {
        name: "std-tools".to_string(),
        base_url: "https://templo-std-tools.herokuapp.com/v1".to_string(),
        requires_authorization: false,
    };

    let registry_dir = config_dir.join("Registry");

    fs::create_dir(&registry_dir)?;

    fs::write(
        registry_dir.join("remote-repos.json"),
        std_error(to_string_pretty(&vec![std_tools_repo]))?,
    )?;

    println!("Config files was created.");

    Ok(())
}
