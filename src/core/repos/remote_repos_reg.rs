use crate::{
    cli::input::namespaces::RemoteRepoNamespace,
    core::path::get_remote_repo_reg_file_path,
    core::utils::errors::{already_exists_error, not_found_error, std_error},
};
use serde_json::{from_str, to_string_pretty};
use std::io::Error;
use std::{fs, path::Path};

pub fn create_regs_file() -> Result<(), Error> {
    let remote_repos_reg = get_remote_repo_reg_file_path()?;
    let namespaces_file_not_exists = !Path::new(&remote_repos_reg).exists();
    if namespaces_file_not_exists {
        fs::write(remote_repos_reg, "[]")?;
    }

    Ok(())
}

pub fn get_reg(namespace_name: &str) -> Result<RemoteRepoNamespace, Error> {
    let repos = get_all_regs()?;
    let namespace = repos
        .into_iter()
        .find(|namespace| namespace.name == namespace_name);
    if let Some(namespace) = namespace {
        Ok(namespace)
    } else {
        Err(not_found_error(&format!(
            "Not is possible to find a namespace named as \"{}\"",
            namespace_name
        )))
    }
}

pub fn get_all_regs() -> Result<Vec<RemoteRepoNamespace>, Error> {
    let remote_repos_filename = get_remote_repo_reg_file_path()?;
    let current_repos_json = fs::read_to_string(&remote_repos_filename)?;
    std_error(from_str(&current_repos_json))
}

pub fn add(repo_registry: RemoteRepoNamespace) -> Result<(), Error> {
    let mut repos = get_all_regs()?;
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
    let repos = get_all_regs()?;

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
    let mut repos = get_all_regs()?;
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

fn update_namespace_file(repos: Vec<RemoteRepoNamespace>) -> Result<(), Error> {
    fs::write(
        get_remote_repo_reg_file_path()?,
        std_error(to_string_pretty(&repos))?,
    )?;
    Ok(())
}
