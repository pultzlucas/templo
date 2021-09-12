use crate::cli::input::args::Args;
use crate::cli::input::{get, get_boolean_input};
use crate::core::path::get_namespaces_file_path;
use crate::core::namespaces::{self, get_namespace, get_saved_namespaces, RemoteRepoNamespace};
use crate::methods::check_flags;
use crate::utils::errors::not_found_error;
use crate::utils::path::pathbuf_to_string;
use std::io::Error;
use tabled::{Style, Table};

pub fn run(args: Args) -> Result<(), Error> {
    let flags = vec!["--local", "--add", "--remove", "--update"];
    check_flags(&args.flags, flags)?;

    if args.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_namespaces_file_path()?));
        return Ok(());
    }

    if args.has_flag("--add") {
        return add_namespace();
    }

    if args.has_flag("--remove") {
        return remove_namespac();
    }

    if args.has_flag("--update") {
        return update_namespace();
    }

    show_saved_namespaces()?;

    Ok(())
}

fn show_saved_namespaces() -> Result<(), Error> {
    let repos = get_saved_namespaces()?;
    let repos_table = Table::new(repos).with(Style::pseudo());

    println!("{}", repos_table);

    Ok(())
}

fn add_namespace() -> Result<(), Error> {
    let name = get("Repo name: ")?;
    let base_url = get("Repo base url: ")?;
    let requires_authorization = get_boolean_input("Repo requires authorization key? [y/n]: ")?;

    let repo_registry = RemoteRepoNamespace {
        name: name.clone(),
        base_url: if base_url.ends_with("/") {
            base_url[..base_url.len() - 1].to_string()
        } else {
            base_url
        },
        requires_authorization,
    };

    namespaces::add(repo_registry.clone())?;

    println!("Remote repo \"{}\" was registered.", repo_registry.name);

    Ok(())
}

fn remove_namespac() -> Result<(), Error> {
    let name = get("Repo name: ")?;

    let yes = get_boolean_input(&format!(
        "You really want to remove the \"{}\" registry? [y/n]: ",
        &name
    ))?;

    if !yes {
        return Ok(());
    }

    namespaces::remove(name.clone())?;

    println!("Remote repo \"{}\" was removed.", name);

    Ok(())
}

fn update_namespace() -> Result<(), Error> {
    println!("Press Enter if you want the field remains the same.");

    let current_name = get("Current repo name: ")?;

    let current_repo = get_namespace(&current_name)?;

    if let Some(current_repo) = current_repo {
        let name = get("New repo name: ")?;
        let base_url = get("New repo base url: ")?;
        let requires_authorization = get_boolean_input("Repo requires authorization key? [y/n]: ")?;

        let repo_updated = RemoteRepoNamespace {
            name: if name.is_empty() {
                current_repo.name
            } else {
                name
            },
            base_url: if base_url.is_empty() {
                current_repo.base_url
            } else {
                base_url
            },
            requires_authorization,
        };

        namespaces::update(&current_name, repo_updated)?;

        println!("Remote repo \"{}\" was updated.", current_name);

        return Ok(());
    }

    Err(not_found_error(&format!(
        "Not found repo registered as \"{}\".",
        current_name
    )))
}
