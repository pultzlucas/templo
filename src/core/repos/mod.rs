mod repository;
pub use repository::*;

use crate::core::utils::errors::{already_exists_error, not_found_error};

use super::path::{get_repo_path, get_root_repos_path};
use std::fs;
use std::io::Error;
use std::path::Path;

pub fn repo_exists(repo_name: &str) -> bool {
    Path::new(&get_repo_path(repo_name).unwrap()).exists()
}

pub fn get_all_repos() -> Result<Vec<String>, Error> {
    let repos_path = get_root_repos_path()?;
    let repos = fs::read_dir(repos_path)?
        .into_iter()
        .map(|repo| repo.unwrap().file_name().to_str().unwrap().to_string());
    Ok(repos.collect())
}

pub fn create_repo(repo_name: &str) -> Result<(), Error> {
    if repo_exists(repo_name) {
        return Err(already_exists_error(&format!(
            "Already exists a repo named as \"{}\".",
            repo_name
        )));
    }

    let repo_path = get_repo_path(repo_name)?;
    fs::create_dir(repo_path)?;
    Ok(())
}

pub fn delete_repo(repo_name: &str) -> Result<(), Error> {
    if !repo_exists(repo_name) {
        return Err(not_found_error(&format!(
            "Repo \"{}\" not exists.",
            repo_name
        )));
    }

    let repo_path = get_repo_path(repo_name)?;
    fs::remove_dir_all(repo_path)?;
    Ok(())
}
