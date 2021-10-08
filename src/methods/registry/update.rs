use std::io::Error;

use crate::{cli::input::{self, command::Command, get_boolean_input}, core::{namespaces::RemoteRepoNamespace, repos::remote_repos_reg::{self, get_reg}}, utils::{errors::invalid_input_error, string::str_to_bool}, write_help};

pub struct Update;

impl Update {
    pub fn help() {
        write_help!("../../../help_files/registry/update.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }
        
        if command.options.is_empty() {
            println!("Press Enter if you want the field remains the same.");
        }

        let current_name = if command.options.is_empty() {
            input::get("Current repo name: ")?
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
            input::get("New repo name: ")?
        } else {
            let input = command.get_opt_by_name("name");
            if let None = input {
                return Err(invalid_input_error("Name of namespace is required."));
            }
            input.unwrap().value.to_owned()
        };

        let base_url = if command.options.is_empty() {
            input::get("New repo base url: ")?
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

        Ok(())
    }
}
