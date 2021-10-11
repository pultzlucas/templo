use std::io::Error;

use crate::{
    cli::input::{self, command::Command, get_boolean_input, namespaces::RemoteRepoNamespace},
    core::repos::remote_repos_reg,
    utils::{errors::invalid_input_error, string::str_to_bool},
    write_help,
};

pub struct Add;

impl Add {
    pub fn help() {
        write_help!("../../../help_files/registry/add.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }

        let name = if command.options.is_empty() {
            input::get("Repo name: ")?
        } else {
            let input = command.get_opt_by_name("name");
            if let None = input {
                return Err(invalid_input_error("Name of namespace is required."));
            }
            input.unwrap().value.to_owned()
        };

        let base_url = if command.options.is_empty() {
            input::get("Repo base url: ")?
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
}
