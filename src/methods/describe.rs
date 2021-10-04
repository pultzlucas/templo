use crate::cli::input::command::Command;
use crate::cli::output::messages::error::INVALID_TEMPLATE_NAME;
use crate::core::namespaces::{get_repo_namespace_obj, NamespaceObject};
use crate::core::repos::Repository;
use crate::paintln;
use crate::utils::errors::invalid_input_error;
use crate::utils::path::pathbuf_to_string;
use std::io::Error;

pub fn run(command: Command) -> Result<(), Error> {
    if command.args.len() == 0 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    // Get template from repository
    let NamespaceObject {
        repo_name,
        template_name,
    } = get_repo_namespace_obj(&command.args[0]);
    let repo = Repository::connect(repo_name)?;
    let template = repo.get_template(&template_name)?;

    paintln!("> {yellow}", template.name);

    // Display template description
    if let Some(description) = template.description {
        println!("{}", description);
    }
    
    // Display template paths
    paintln!("{gray}", "[paths]");
    template
        .paths
        .into_iter()
        .for_each(|path| println!("{}", pathbuf_to_string(path.path)));

    Ok(())
}
