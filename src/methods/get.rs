use crate::cli::input::command::Command;
use crate::core::namespaces::parse_namespace_to_raw_url;
use crate::core::repos::remote_repos_reg::get_reg;
use crate::core::repos::Repository;
use crate::core::requester::{str_is_url, validate_url};
use crate::core::template::getter::get_remote_template;
use crate::{
    cli::output::messages::error::TEMPLATE_ALREADY_EXISTS,
    paintln,
    utils::errors::{already_exists_error, invalid_input_error},
};
use std::{io::Error, str, time::Instant};

pub async fn run(command: Command) -> Result<(), Error> {
    if command.args.len() < 1 {
        return Err(invalid_input_error("The template url must be specified."));
    }

    let start = Instant::now(); // start timing process

    let url = if str_is_url(&command.args[0]) {
        validate_url(&command.args[0])?.to_string()
    } else {
        let template_url_path = command.args[0].clone();
        let url = parse_namespace_to_raw_url(template_url_path)?;
        validate_url(&url)?.to_string()
    };

    let namespace_name = command.args[0].split("/").collect::<Vec<&str>>()[0];
    let namespace = get_reg(namespace_name)?;

    let key = if namespace.requires_authorization {
        let key = command.get_opt_by_name("key");
        if let None = key {
            return Err(invalid_input_error(
                "This remote repo requires authorization key. Add --key=<key> option.",
            ));
        }
        Some(key.unwrap().value.clone())
    } else {
        None
    };

    paintln!("{gray}", "[getting template]");
    let response = get_remote_template(&url, key).await?;

    if let Some(msg) = response.message {
        println!("{}", msg);
    }

    let template = response.template;
    let repo_name = if command.args.len() > 1 {
        command.args[1].clone()
    } else {
        "main".to_string()
    };

    let repo = Repository::connect(repo_name)?;

    //check if a template with the same name already exists in repo
    if repo.template_exists(&template.name) {
        return Err(already_exists_error(TEMPLATE_ALREADY_EXISTS));
    }

    repo.save_template(template.clone())?;
    println!(
        "Template \"{}\" was installed in \"{}\" repo.",
        template.name, repo.name
    );

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
