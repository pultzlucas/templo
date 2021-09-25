use crate::cli::input;
use crate::cli::input::command::Command;
use crate::cli::output::messages::error::INVALID_TEMPLATE_NAME;
use crate::core::repo;
use crate::core::template::maker::make_template;
use crate::methods::check_flags;
use crate::utils::errors::invalid_input_error;
use std::io::Error;
use std::time::Instant;

pub fn run(command: Command) -> Result<(), Error> {
    repo::create()?;

    let flags = vec!["--name", "--description"];
    check_flags(&command.flags, flags)?;

    if command.has_flag("--description") {
        if command.args.len() < 1 {
            return Err(invalid_input_error(
                "Current template name must be specified.",
            ));
        }

        let template_name = &command.args[0];
        let template = repo::get_template(template_name)?;

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

        repo::update_template_description(template_name, new_description)?;

        println!("Template \"{}\" was updated.", template_name);
        return Ok(())
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

        repo::update_template_name(old_template_name, new_template_name.clone())?;

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
    let template_name = command.args[0].clone();

    let description = repo::get_template(&template_name)?.description;
    let new_template = make_template(template_name.clone(), description, ".")?;
    repo::update_template_content(template_name.clone(), new_template)?;

    println!("Template \"{}\" was updated.", template_name);

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
