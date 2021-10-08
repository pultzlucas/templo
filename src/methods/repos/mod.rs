use crate::{
    cli::input::command::Command, core::repos::get_all_repos, utils::errors::invalid_input_error,
    write_help,
};
use std::io::Error;

mod create;
mod delete;

use create::Create;
use delete::Delete;

pub struct Repos;

impl Repos {
    pub fn help() {
        write_help!("../../../help_files/repos/mod.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() && command.submethod.is_none() {
            Self::help();
            return Ok(());
        }

        if let Some(submethod) = command.submethod.clone() {
            return match submethod.as_str() {
                "create" => Create::run(command),
                "del" => Delete::run(command),
                _ => {
                    return Err(invalid_input_error(&format!(
                        "Invalid submethod \"{}\".",
                        submethod
                    )))
                }
            };
        }

        let repos = get_all_repos()?;
        // print each repo name
        repos.iter().for_each(|repo| {
            println!("{}", repo);
        });

        Ok(())
    }
}
