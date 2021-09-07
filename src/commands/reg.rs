use crate::cli::input::args::Args;
use crate::cli::input::{get, get_boolean_input};
use crate::core::config;
use crate::core::config::registry::RemoteRepoRegistry;
use crate::core::path::get_config_path;
use crate::utils::path::pathbuf_to_string;
use std::io::Error;
use tabled::{Style, Table};

pub fn run(args: Args) -> Result<(), Error> {
    config::create_files()?;

    if args.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_config_path()?));
        return Ok(());
    }

    if args.has_flag("--add") {
        return add_registry_remote_repo();
    }

    if args.has_flag("--remove") {
        return remove_registry_remote_repo();
    }

    if args.has_flag("--update") {
        return update_registry_remote_repo();
    }

    show_registered_remote_repos()?;

    Ok(())
}

fn show_registered_remote_repos() -> Result<(), Error> {
    let repos = config::registry::repo::get_registered_repos()?;
    let repos_table = Table::new(repos).with(Style::pseudo());

    println!("{}", repos_table);

    Ok(())
}

fn add_registry_remote_repo() -> Result<(), Error> {
    let name = get("Repo name: ")?;
    let url = get("Repo url: ")?;
    let requires_authorization = get_boolean_input("Repo requires authorization key? [y/n]: ")?;

    let repo_registry = RemoteRepoRegistry {
        name: name.clone(),
        url,
        requires_authorization,
    };

    config::registry::repo::add(repo_registry.clone())?;

    println!("Remote repo \"{}\" was registered.", repo_registry.name);

    Ok(())
}

fn remove_registry_remote_repo() -> Result<(), Error> {
    let name = get("Repo name: ")?;

    let yes = get_boolean_input(&format!(
        "You really want to remove the \"{}\" registry? [y/n]: ",
        &name
    ))?;

    if !yes {
        return Ok(());
    }

    config::registry::repo::remove(name.clone())?;

    println!("Remote repo \"{}\" was removed.", name);

    Ok(())
}

fn update_registry_remote_repo() -> Result<(), Error> {
    println!("Press Enter if you want the field remains the same.");

    let current_name = get("Current repo name: ")?;
    let name = get("New repo name: ")?;
    let url = get("New repo url: ")?;
    let requires_authorization = get_boolean_input("Repo requires authorization key? [y/n]: ")?;

    let repo_updated = RemoteRepoRegistry {
        name: name.clone(),
        url,
        requires_authorization,
    };

    config::registry::repo::update(&current_name, repo_updated)?;

    println!("Remote repo \"{}\" was updated.", current_name);

    Ok(())
}
