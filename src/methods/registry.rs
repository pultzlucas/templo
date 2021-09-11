use crate::cli::input::args::Args;
use crate::cli::input::{get, get_boolean_input};
use crate::core::config;
use crate::core::config::registry::remote_repos::get_repo_registry;
use crate::core::config::registry::RemoteRepoRegistry;
use crate::core::path::get_config_path;
use crate::methods::check_flags;
use crate::utils::errors::not_found_error;
use crate::utils::path::pathbuf_to_string;
use std::io::Error;
use tabled::{Style, Table};

pub fn run(args: Args) -> Result<(), Error> {
    config::create_files()?;

    let flags = vec!["--local", "--add", "--remove", "--update"];
    check_flags(&args.flags, flags)?;

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
    let repos = config::registry::remote_repos::get_registered_repos()?;
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

    config::registry::remote_repos::add(repo_registry.clone())?;

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

    config::registry::remote_repos::remove(name.clone())?;

    println!("Remote repo \"{}\" was removed.", name);

    Ok(())
}

fn update_registry_remote_repo() -> Result<(), Error> {
    println!("Press Enter if you want the field remains the same.");

    let current_name = get("Current repo name: ")?;

    let current_repo = get_repo_registry(&current_name)?;

    if let Some(current_repo) = current_repo {
        let name = get("New repo name: ")?;
        let url = get("New repo url: ")?;
        let requires_authorization = get_boolean_input("Repo requires authorization key? [y/n]: ")?;

        let repo_updated = RemoteRepoRegistry {
            name: if name.is_empty() {
                current_repo.name
            } else {
                name
            },
            url: if url.is_empty() {
                current_repo.url
            } else {
                url
            },
            requires_authorization
        };

        config::registry::remote_repos::update(&current_name, repo_updated)?;

        println!("Remote repo \"{}\" was updated.", current_name);

        return Ok(());
    }

    Err(not_found_error(&format!(
        "Not found repo registered as \"{}\".",
        current_name
    )))
}
