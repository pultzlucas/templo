use crate::core::config::get_config_path;
use crate::core::config::repos::LocalRepoRegistry;
use crate::utils::errors::{already_exists_error, not_found_error, std_error};
use serde_json;
use std::fs;
use std::io::Error;
use std::path::PathBuf;
pub fn get_repo_registry(repo_name: &str) -> Result<Option<LocalRepoRegistry>, Error> {
    let repos = get_registered_repos()?;
    Ok(repos.into_iter().find(|repo| repo.name == repo_name))
}

pub fn get_registered_repos() -> Result<Vec<LocalRepoRegistry>, Error> {
    let local_repos_filename = get_local_repos_filename()?;
    let current_repos_json = fs::read_to_string(&local_repos_filename)?;
    std_error(serde_json::from_str(&current_repos_json))
}

pub fn add(repo: LocalRepoRegistry) -> Result<(), Error> {
    let mut repos = get_registered_repos()?;
    let name_already_is_used = repos.iter().any(|repo| repo.name == repo.name);

    if name_already_is_used {
        return Err(already_exists_error(&format!(
            "Already exists a remote repo registered as \"{}\"",
            repo.name
        )));
    }

    repos.push(repo);
    update_local_repos(repos)?;

    Ok(())
}

pub fn remove(repo_name: String) -> Result<(), Error> {
    let repos = get_registered_repos()?;

    if let None = repos.iter().find(|repo| repo_name == repo.name) {
        return Err(not_found_error(&format!(
            "Repo \"{}\" not is registered.",
            repo_name
        )));
    }

    let repos_new: Vec<LocalRepoRegistry> = repos
        .into_iter()
        .filter(|repo| repo_name != repo.name)
        .collect();

    update_local_repos(repos_new)?;

    Ok(())
}

pub fn update(current_name: &str, repo_updated: LocalRepoRegistry) -> Result<(), Error> {
    let mut repos = get_registered_repos()?;
    let repo = repos
        .iter()
        .rposition(|reg_repo| reg_repo.name == current_name);

    if let Some(repo_idx) = repo {
        repos.remove(repo_idx);
        repos.push(repo_updated);
        update_local_repos(repos)?;

        return Ok(());
    }

    Err(not_found_error(&format!(
        "Repo \"{}\" not is registered.",
        repo_updated.name
    )))
}

fn get_local_repos_filename() -> Result<PathBuf, Error> {
    Ok(get_config_path()?.join("Repos").join("local.json"))
}

fn update_local_repos(repos: Vec<LocalRepoRegistry>) -> Result<(), Error> {
    fs::write(
        get_local_repos_filename()?,
        std_error(serde_json::to_string_pretty(&repos))?,
    )?;
    Ok(())
}
