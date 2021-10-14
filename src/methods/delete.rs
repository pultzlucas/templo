use crate::cli::input::{command::Command, namespaces::get_repo_namespace_obj};
use crate::cli::output::messages::error::INVALID_TEMPLATE_NAME;
use crate::core::repos::Repository;
use crate::core::utils::errors::invalid_input_error;
use crate::write_help;
use std::{io::Error, time::Instant};

pub struct Delete;

impl Delete {
    pub fn help() {
        write_help!("../../help_files/delete.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }

        if command.args.len() < 1 {
            return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
        }

        let start = Instant::now(); // start timing process
        let templates_name = &command.args[0..];

        // Delete templates
        for temp_name in templates_name.into_iter() {
            let namespace = get_repo_namespace_obj(temp_name);
            let repo = Repository::connect(namespace.repo_name.clone())?;
            repo.delete_template(&namespace.template_name)?;
            println!(
                "Template \"{}\" was deleted from \"{}\".",
                namespace.template_name, namespace.repo_name
            );
        }

        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));

        Ok(())
    }
}
