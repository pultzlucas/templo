use crate::cli::input;
use crate::cli::input::command::Command;
use crate::cli::output::messages::error::INVALID_TEMPLATE_NAME;
use crate::core::namespaces::{get_repo_namespace_obj, NamespaceObject};
use crate::core::repos::Repository;
use crate::core::template::maker::make_template;
use crate::methods::check_flags;
use crate::utils::errors::invalid_input_error;
use crate::write_help;
use std::io::Error;
use std::time::Instant;

pub struct Update;

impl Update {
    pub fn help() {
        write_help!("../../help_files/update.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }
        
        let flags = vec!["--name", "--description"];
        check_flags(&command.flags, flags)?;

        let template_namespace = &command.args[0];
        let NamespaceObject {
            repo_name,
            template_name,
        } = get_repo_namespace_obj(template_namespace);
        let repo = Repository::connect(repo_name)?;

        if command.has_flag("--description") {
            if command.args.len() < 1 {
                return Err(invalid_input_error(
                    "Current template name must be specified.",
                ));
            }

            let template_name = &command.args[0];
            let template = repo.get_template(template_name)?;

            if let Some(description) = template.description {
                println!("Current description: {}", description);
            } else {
                println!("This template not has a description yet.")
            }

            // Get template description
            let new_description = input::get("New template description: ")?;
            let new_description = if new_description.is_empty() {
                None
            } else {
                Some(new_description)
            };

            repo.update_template_description(template_name, new_description)?;

            println!("Template \"{}\" was updated.", template_name);
            return Ok(());
        }

        if command.has_flag("--name") {
            if command.args.len() < 1 {
                return Err(invalid_input_error(
                    "Current template name must be specified.",
                ));
            }

            let old_template_name = &command.args[0];
            let new_template_name = input::get("New template name: ")?;

            if new_template_name.is_empty() {
                return Err(invalid_input_error("New template name must be specified."));
            }

            repo.update_template_name(old_template_name, new_template_name.clone())?;

            println!(
                "Template \"{}\" name was changed to \"{}\".",
                old_template_name, new_template_name
            );

            return Ok(());
        }

        if command.args.len() < 1 {
            return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
        }

        let start = Instant::now(); // start timing process

        let directory = if command.args.len() > 1 {
            &command.args[1]
        } else {
            "."
        };

        let description = repo.get_template(&template_name)?.description;
        let new_template = make_template(template_name.clone(), description, directory)?;
        repo.update_template_content(template_name.clone(), new_template)?;

        println!("Template \"{}\" was updated.", template_name);

        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));

        Ok(())
    }
}
