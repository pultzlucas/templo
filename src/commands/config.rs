use crate::cli::input::args::Args;
use crate::cli::input::{get, get_valid_input};
use crate::core::config::{self, RemoteRepoRegistry};
use crate::core::path::get_config_path;
use crate::utils::path::pathbuf_to_string;
use std::io::Error;

pub fn run(args: Args) -> Result<(), Error> {
    config::create_files()?;

    if args.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_config_path()?));
        return Ok(());
    }

    if args.has_flag("--add") {
        match args.inputs[0].as_str() {
            "repo:remote" => add_registry_remote_repo()?,
            _ => return Ok(()),
        }
    }

    if args.has_flag("--remove") {
        match args.inputs[0].as_str() {
            "repo:remote" => remove_registry_remote_repo()?,
            _ => return Ok(()),
        }
    }

    if args.has_flag("--update") {
        match args.inputs[0].as_str() {
            "repo:remote" => update_registry_remote_repo()?,
            _ => return Ok(()),
        }
    }

    Ok(())
}

fn add_registry_remote_repo() -> Result<(), Error> {
    let name = get("Repo name: ")?;
    let url = get("Repo url: ")?;
    let authorization = get_valid_input("Repo authorization key (null): ", None, |input| {
        input != "n" && input != "y" && input != "N" && input != "Y"
    })?;

    let registry = RemoteRepoRegistry {
        name: name.clone(),
        url,
        require_authorization: authorization == "y"
    };

    config::repos::remote::add(registry)?;

    println!("Repo \"{}\" was registered.", name);

    Ok(())
}

fn remove_registry_remote_repo() -> Result<(), Error> {
    let name = get("Repo name: ")?;
    config::repos::remote::remove(name.clone())?;

    println!("Repo \"{}\" was removed.", name);

    Ok(())
}

fn update_registry_remote_repo() -> Result<(), Error> {
    let name = get("Repo name: ")?;
    let url = get("Repo url: ")?;
    let authorization = get_valid_input("Repo authorization key (null): ", None, |input| {
        input != "n" && input != "y" && input != "N" && input != "Y"
    })?;

    let repo_updated = RemoteRepoRegistry {
        name: name.clone(),
        url,
        require_authorization: authorization == "y"
    };

    config::repos::remote::update(repo_updated)?;

    println!("Repo \"{}\" was updated.", name);

    Ok(())
}
