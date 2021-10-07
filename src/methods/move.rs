use crate::{cli::input::command::Command, core::{
        namespaces::{get_repo_namespace_obj, NamespaceObject},
        repos::Repository,
    }, utils::errors::invalid_input_error, write_help};
use std::io::Error;

pub struct Move;

impl Move {
    pub fn help() {
        write_help!("../../help_files/move.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }
        
        if command.args.len() < 1 {
            return Err(invalid_input_error("Template name must be specified."));
        }
    
        if command.args.len() < 2 {
            return Err(invalid_input_error("Repo name must be specified."));
        }
        let template_full_name = &command.args[0];
        let recipient_repo_name = &command.args[1];
    
        let NamespaceObject {
            repo_name,
            template_name,
        } = get_repo_namespace_obj(&template_full_name);
    
        let repo_sender = Repository::connect(repo_name.clone())?;
        let repo_recipient = Repository::connect(recipient_repo_name.to_owned())?;
        repo_sender.move_template_to(&template_name, repo_recipient.clone())?;
    
        println!("Template \"{}\" was moved to \"{}\" repo.", template_name, repo_recipient.name);
        Ok(())
    }
}