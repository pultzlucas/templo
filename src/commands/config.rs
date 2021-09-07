use crate::cli::input::args::Args;
use crate::cli::input::{get, get_boolean_input};
use crate::core::config::{self, RemoteRepoRegistry};
use crate::core::path::get_config_path;
use crate::utils::errors::invalid_input_error;
use crate::utils::path::pathbuf_to_string;
use std::io::Error;
use tabled::{Style, Table};

pub fn run(args: Args) -> Result<(), Error> {
    config::create_files()?;
    
    if args.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_config_path()?));
        return Ok(());
    }
    
    if args.inputs.len() == 0 {
        return Ok(());
    }

    if args.has_flag("--add") {
        return match args.inputs[0].as_str() {
            "repos:remote" => add_registry_remote_repo(),
            "repos:local" => add_registry_local_repo(),
            _ => Ok(()),
        };
    }

    if args.has_flag("--remove") {
        return match args.inputs[0].as_str() {
            "repos:remote" => remove_registry_remote_repo(),
            "repos:local" => remove_registry_local_repo(),
            _ => Ok(()),
        };
    }

    if args.has_flag("--update") {
        return match args.inputs[0].as_str() {
            "repos:remote" => update_registry_remote_repo(),
            "repos:local" => update_registry_local_repo(),
            _ => Ok(()),
        };
    }

    match args.inputs[0].as_str() {
        "repos:remote" => show_registered_remote_repos()?,
        "repos:local"  => show_registered_local_repos()?,
        _ => return Err(invalid_input_error("Invalid config option")),
    }

    Ok(())
}

// LOCAL REPOS

fn show_registered_local_repos() -> Result<(), Error> {


    Ok(())
}

fn add_registry_local_repo() -> Result<(), Error> {


    Ok(())
}

fn remove_registry_local_repo() -> Result<(), Error> {


    Ok(())
}

fn update_registry_local_repo() -> Result<(), Error> {


    Ok(())
}


// REMOTE REPOS

fn show_registered_remote_repos() -> Result<(), Error> {
    let repos = config::repos::remote::get_repos_registered()?;
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

    config::repos::remote::add(repo_registry.clone())?;

    println!("Repo \"{}\" was registered.", repo_registry.name);

    Ok(())
}

fn remove_registry_remote_repo() -> Result<(), Error> {
    let name = get("Repo name: ")?;

    let yes = get_boolean_input(&format!("You really want to remove the \"{}\" registry? [y/n]: ", &name))?;
    if !yes {
        return Ok(());
    }

    config::repos::remote::remove(name.clone())?;

    println!("Repo \"{}\" was removed.", name);

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

    config::repos::remote::update(&current_name, repo_updated)?;

    println!("Repo \"{}\" was updated.", current_name);

    Ok(())
}
