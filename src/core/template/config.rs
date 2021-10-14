use crate::core::utils::errors::std_error;
use crate::core::utils::path::str_to_pathbuf;
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConfigArg {
    pub key: String,
    pub query: String,
    pub about: Option<String>,
    pub default: Option<String>,
}

pub fn get_config_args(directory_path: &str) -> Result<Option<Vec<ConfigArg>>, Error> {
    let args_filename = get_config_folder_path(directory_path).join("args.json");

    if !Path::new(&args_filename).exists() {
        return Ok(None);
    }

    let args_string = fs::read_to_string(args_filename)?;
    let config_args: Vec<ConfigArg> = std_error(from_str(&args_string))?;
    Ok(Some(config_args))
}

fn get_config_folder_path(directory_path: &str) -> PathBuf {
    str_to_pathbuf(directory_path).join("TemplateConfig")
}
