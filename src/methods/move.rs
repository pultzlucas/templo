use crate::{
    cli::input::command::Command,
    core::{namespaces::get_repo_namespace_obj, repos::Repository},
    utils::errors::invalid_input_error,
};
use std::io::Error;

pub fn run(command: Command) -> Result<(), Error> {
    if command.args.len() < 1 {
        return Err(invalid_input_error("Template name must be specified."));
    }

    if command.args.len() < 2 {
        return Err(invalid_input_error("Repo name must be specified."));
    }
    let template_full_name = &command.args[0];
    let recipient_repo_name = &command.args[1];

    let template_namespace = get_repo_namespace_obj(&template_full_name);

    let repo_sender = Repository::connect(template_namespace.repo_name)?;
    let repo_recipient = Repository::connect(recipient_repo_name.to_owned())?;

    repo_sender.move_template_to(&template_namespace.template_name, repo_recipient)?;

    Ok(())
}
