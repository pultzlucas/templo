
use crate::cli::input::command::Command;
use crate::core::path::get_remote_repo_reg_file_path;
use crate::core::repos::remote_repos_reg::{create_regs_file, get_all_regs};
use crate::methods::check_flags;
use crate::utils::errors::invalid_input_error;
use crate::utils::path::pathbuf_to_string;
use crate::write_help;
use std::io::Error;
use tabled::{Style, Table};

mod update;
mod add;
mod remove;

use update::Update;
use add::Add;
use remove::Remove;

pub struct Registry;

impl Registry {
    pub fn help() {
        write_help!("../../../help_files/registry/mod.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() && command.submethod.is_none() {
            Self::help();
            return Ok(());
        }

        create_regs_file()?;

        let flags = vec!["--local", "-y"];
        check_flags(&command.flags, flags)?;

        if command.has_flag("--local") {
            println!("{}", pathbuf_to_string(get_remote_repo_reg_file_path()?));
            return Ok(());
        }

        if let Some(submethod) = command.submethod.as_ref() {
            return match submethod.as_str() {
                "add" => Add::run(command),
                "remove" => Remove::run(command),
                "update" => Update::run(command),
                _ => Err(invalid_input_error(&format!(
                    "Invalid namespace method \"{}\"",
                    submethod
                ))),
            };
        }

        show_saved_namespaces()?;

        Ok(())
    }
}

fn show_saved_namespaces() -> Result<(), Error> {
    let repos = get_all_regs()?;

    if repos.is_empty() {
        println!("No remote repos registered.");
        return Ok(());
    }

    let repos_table = Table::new(repos).with(Style::pseudo());

    print!("{}", repos_table);

    Ok(())
}
