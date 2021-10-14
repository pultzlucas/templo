use std::io::Error;

use crate::{
    cli::input::{self, command::Command, get_boolean_input},
    core::repos::remote_repos_reg,
    core::utils::errors::invalid_input_error,
    write_help,
};

pub struct Remove;

impl Remove {
    pub fn help() {
        write_help!("../../../help_files/registry/remove.json");
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

        let yes = if command.has_flag("-y") || command.has_flag("--yes") {
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
}
