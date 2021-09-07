use super::{get_config_path, RemoteRepoRegistry};
use crate::utils::errors::{already_exists_error, not_found_error, std_error};
use serde_json;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub mod remote {
    use super::*;

    pub fn get_repo_registry(repo_name: &str) -> Result<Option<RemoteRepoRegistry>, Error> {
        let repos = get_repos_registered()?;
        Ok(repos.into_iter().find(|repo| repo.name == repo_name))
    }

    pub fn get_repos_registered() -> Result<Vec<RemoteRepoRegistry>, Error> {
        let remote_repos_filename = get_remote_repos_filename()?;
        let current_repos_json = fs::read_to_string(&remote_repos_filename)?;
        std_error(serde_json::from_str(&current_repos_json))
    }

    pub fn add(repo_registry: RemoteRepoRegistry) -> Result<(), Error> {
        let mut repos = get_repos_registered()?;
        let name_already_is_used = repos.iter().any(|repo| repo.name == repo_registry.name);

        if name_already_is_used {
            return Err(already_exists_error(&format!(
                "Already exists a remote repo registered as \"{}\"",
                repo_registry.name
            )));
        }

        repos.push(repo_registry);
        update_remote_repos(repos)?;

        Ok(())
    }

    pub fn remove(repo_name: String) -> Result<(), Error> {
        let repos = get_repos_registered()?;

        if let None = repos.iter().find(|repo| repo_name == repo.name) {
            return Err(not_found_error(&format!(
                "Repo \"{}\" not is registered.",
                repo_name
            )));
        }

        let repos_new: Vec<RemoteRepoRegistry> = repos
            .into_iter()
            .filter(|repo| repo_name != repo.name)
            .collect();

        update_remote_repos(repos_new)?;

        Ok(())
    }

    pub fn update(current_name: &str, repo_updated: RemoteRepoRegistry) -> Result<(), Error> {
        let mut repos = get_repos_registered()?;
        let repo = repos
            .iter()
            .rposition(|reg_repo| reg_repo.name == current_name);

        if let Some(repo_idx) = repo {
            repos.remove(repo_idx);
            repos.push(repo_updated);
            update_remote_repos(repos)?;

            return Ok(());
        }

        Err(not_found_error(&format!(
            "Repo \"{}\" not is registered.",
            repo_updated.name
        )))
    }

    fn get_remote_repos_filename() -> Result<PathBuf, Error> {
        Ok(get_config_path()?.join("Repos").join("remote.json"))
    }

    fn update_remote_repos(repos: Vec<RemoteRepoRegistry>) -> Result<(), Error> {
        fs::write(
            get_remote_repos_filename()?,
            std_error(serde_json::to_string_pretty(&repos))?,
        )?;
        Ok(())
    }
}
