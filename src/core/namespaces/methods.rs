use crate::utils::errors::{already_exists_error, not_found_error};
use super::{RemoteRepoNamespace, update_namespace_file, get_saved_namespaces};
use std::io::Error;

pub fn add(repo_registry: RemoteRepoNamespace) -> Result<(), Error> {
    let mut repos = get_saved_namespaces()?;
    let name_already_is_used = repos.iter().any(|repo| repo.name == repo_registry.name);

    if name_already_is_used {
        return Err(already_exists_error(&format!(
            "Already exists a remote repo registered as \"{}\"",
            repo_registry.name
        )));
    }

    repos.push(repo_registry);
    update_namespace_file(repos)?;

    Ok(())
}

pub fn remove(repo_name: String) -> Result<(), Error> {
    let repos = get_saved_namespaces()?;

    if let None = repos.iter().find(|repo| repo_name == repo.name) {
        return Err(not_found_error(&format!(
            "Repo \"{}\" not is registered.",
            repo_name
        )));
    }

    let repos_new: Vec<RemoteRepoNamespace> = repos
        .into_iter()
        .filter(|repo| repo_name != repo.name)
        .collect();

    update_namespace_file(repos_new)?;

    Ok(())
}

pub fn update(current_name: &str, repo_updated: RemoteRepoNamespace) -> Result<(), Error> {
    let mut repos = get_saved_namespaces()?;
    let repo = repos
        .iter()
        .rposition(|reg_repo| reg_repo.name == current_name);

    if let Some(repo_idx) = repo {
        repos.remove(repo_idx);
        repos.push(repo_updated);
        update_namespace_file(repos)?;

        return Ok(());
    }

    Err(not_found_error(&format!(
        "Repo \"{}\" not is registered.",
        repo_updated.name
    )))
}