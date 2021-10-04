use crate::cli::input::command::Command;
use crate::cli::input::{get, get_boolean_input};
use crate::core::namespaces::RemoteRepoNamespace;
use crate::core::repos::remote_repos_reg::{
    self, create_regs_file, get_reg, get_all_regs,
};
use crate::core::path::get_remote_repo_reg_file_path;
use crate::methods::check_flags;
use crate::utils::errors::invalid_input_error;
use crate::utils::path::pathbuf_to_string;
use crate::utils::string::str_to_bool;
use std::io::Error;
use tabled::{Style, Table};

pub fn run(command: Command) -> Result<(), Error> {
    create_regs_file()?;

    let flags = vec!["--local", "-y"];
    check_flags(&command.flags, flags)?;

    if command.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_remote_repo_reg_file_path()?));
        return Ok(());
    }

    if let Some(submethod) = command.submethod.as_ref() {
        return match submethod.as_str() {
            "add" => add_namespace(command),
            "remove" => remove_namespace(command),
            "update" => update_namespace(command),
            _ => Err(invalid_input_error(&format!("Invalid namespace method \"{}\"", submethod)))
        };
    }
    
    show_saved_namespaces()?;

    Ok(())
}

fn show_saved_namespaces() -> Result<(), Error> {
    let repos = get_all_regs()?;

    if repos.is_empty() {
        println!("No remote repos registered.");
        return Ok(())
    }

    let repos_table = Table::new(repos).with(Style::pseudo());

    print!("{}", repos_table);

    Ok(())
}

fn add_namespace(command: Command) -> Result<(), Error> {
    let name = if command.options.is_empty() {
        get("Repo name: ")?
    } else {
        let input = command.get_opt_by_name("name");
        if let None = input {
            return Err(invalid_input_error("Name of namespace is required."));
        }
        input.unwrap().value.to_owned()
    };

    let base_url = if command.options.is_empty() {
        get("Repo base url: ")?
    } else {
        let base_url_input = command.get_opt_by_name("base-url");
        if let None = base_url_input {
            return Err(invalid_input_error("Base url of namespace is required."));
        }
        base_url_input.unwrap().value.to_owned()
    };

    let requires_authorization = if command.options.is_empty() {
        get_boolean_input("Repo requires authorization key? [y/n]: ")?
    } else {
        let input = command.get_opt_by_name("req-auth");
        if let None = input {
            return Err(invalid_input_error(
                "The requires authorization key information of namespace is required.",
            ));
        }
        str_to_bool(&input.unwrap().value)
    };

    let repo_registry = RemoteRepoNamespace {
        name: name.clone(),
        base_url: if base_url.ends_with("/") {
            base_url[..base_url.len() - 1].to_string()
        } else {
            base_url
        },
        requires_authorization,
    };

    remote_repos_reg::add(repo_registry.clone())?;

    println!("Remote repo \"{}\" was registered.", repo_registry.name);

    Ok(())
}

fn remove_namespace(command: Command) -> Result<(), Error> {
    let name = if command.options.is_empty() {
        get("Repo name: ")?
    } else {
        let input = command.get_opt_by_name("name");
        if let None = input {
            return Err(invalid_input_error("Name of namespace is required."));
        }
        input.unwrap().value.to_owned()
    };

    let yes = if command.has_flag("-y") {
        true
    } else {
        get_boolean_input(&format!(
            "You really want to remove the \"{}\" registry? [y/n]: ",
            &name
        ))?
    };

    if !yes {
        return Ok(());
    }

    remote_repos_reg::remove(name.clone())?;

    println!("Remote repo \"{}\" was removed.", name);

    Ok(())
}

fn update_namespace(command: Command) -> Result<(), Error> {
    if command.options.is_empty() {
        println!("Press Enter if you want the field remains the same.");
    }

    let current_name = if command.options.is_empty() {
        get("Current repo name: ")?
    } else {
        let input = command.get_opt_by_name("current-name");
        if let None = input {
            return Err(invalid_input_error(
                "Current name of namespace is required.",
            ));
        }
        input.unwrap().value.to_owned()
    };

    let current_repo = get_reg(&current_name)?;

    let name = if command.options.is_empty() {
        get("New repo name: ")?
    } else {
        let input = command.get_opt_by_name("name");
        if let None = input {
            return Err(invalid_input_error("Name of namespace is required."));
        }
        input.unwrap().value.to_owned()
    };

    let base_url = if command.options.is_empty() {
        get("New repo base url: ")?
    } else {
        let input = command.get_opt_by_name("base-url");
        if let None = input {
            return Err(invalid_input_error("Base url of namespace is required."));
        }
        input.unwrap().value.to_owned()
    };

    let requires_authorization = if command.options.is_empty() {
        get_boolean_input("Repo requires authorization key? [y/n]: ")?
    } else {
        let input = command.get_opt_by_name("req-auth");
        if let None = input {
            return Err(invalid_input_error(
                "The requires authorization key information of namespace is required.",
            ));
        }
        str_to_bool(&input.unwrap().value)
    };

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

    remote_repos_reg::update(&current_name, repo_updated)?;

    println!("Remote repo \"{}\" was updated.", current_name);

    return Ok(());
}
