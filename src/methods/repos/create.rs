use std::io::Error;

use crate::{cli::input::command::Command, core::repos::create_repo, utils::errors::invalid_input_error, write_help};

pub struct Create;

impl Create {
    pub fn help() {
        write_help!("../../../help_files/repos/create.json");
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
        create_repo(repo_name)?;

        println!("Repo \"{}\" was created.", repo_name);
        Ok(())
    }
}
