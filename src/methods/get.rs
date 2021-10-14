use crate::cli::input::command::Command;
use crate::core::http::validate_url;
use crate::core::repos::Repository;
use crate::core::template::getter::get_remote_template;
use crate::write_help;
use crate::{
    core::utils::errors::{already_exists_error, invalid_input_error},
    paintln,
};
use std::{io::Error, str, time::Instant};

pub struct Get;

impl Get {
    pub fn help() {
        write_help!("../../help_files/get.json");
    }

    pub async fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }

        let start = Instant::now(); // start timing process

        if command.args.is_empty() {
            return Err(invalid_input_error("The template url must be specified."));
        }

        let url = &command.args[0];
        validate_url(&url)?;

        let key = if command.has_option("auth") {
            let key = command.get_opt_by_name("auth").unwrap();
            Some(key.value.clone())
        } else {
            None
        };

        let repo_name = if command.args.len() > 1 {
            command.args[1].clone()
        } else {
            "main".to_string()
        };

        let repo = Repository::connect(repo_name)?;
        
        paintln!("{gray}", "[getting template...]");
        let response = get_remote_template(&url, key).await?;

        let template = response.template;

        //check if a template with the same name already exists in repo
        if repo.has_template(&template.name) {
            return Err(already_exists_error(&format!(
                "Template \"{}\" already exists in \"{}\" repo.",
                &template.name, &repo.name
            )));
        }

        repo.save_template(template.clone())?;

        if let Some(msg) = response.message {
            println!("{}", msg);
        }

        println!(
            "Template \"{}\" was saved in \"{}\" repo.",
            template.name, repo.name
        );

        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));

        Ok(())
    }
}
