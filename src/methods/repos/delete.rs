use std::io::Error;

use crate::{
    cli::input::{command::Command, get_boolean_input},
    core::repos::{delete_repo, repo_exists},
    core::utils::errors::{invalid_input_error, not_found_error},
    write_help,
};

pub struct Delete;

impl Delete {
    pub fn help() {
        write_help!("../../../help_files/repos/delete.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }

        if command.args.len() < 2 {
            return Err(invalid_input_error("Repo name must be specified."));
        }

        let repo_name = &command.args[1];
        if !repo_exists(repo_name) {
            if !repo_exists(repo_name) {
                return Err(not_found_error(&format!(
                    "Repo \"{}\" not exists.",
                    repo_name
                )));
            }
        }

        let yes = if command.has_flag("-y").clone() {
            true
        } else {
            get_boolean_input(&format!("Do you really want to delete \"{}\" repo and all the templates that exists inside within it? [y/n]: ", repo_name))?
        };

        if yes {
            println!("Repo \"{}\" was deleted.", repo_name);
            delete_repo(repo_name)?;
            return Ok(());
        }
        Ok(())
    }
}
