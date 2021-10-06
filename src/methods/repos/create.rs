use std::io::Error;

use crate::{cli::input::command::Command, core::repos::create_repo, utils::errors::invalid_input_error};

pub struct Create;

impl Create {
    pub fn run(command: Command) -> Result<(), Error> {
        if command.args.len() < 2 {
            return Err(invalid_input_error("Repo name must be specified."));
        }

        let repo_name = &command.args[1];
        create_repo(repo_name)?;

        println!("Repo \"{}\" was created.", repo_name);
        Ok(())
    }
}
