use crate::cli::input::command::Command;
use crate::core::namespaces::{get_namespace, parse_to_raw_url};
use crate::core::repo;
use crate::core::requester::{
    build_request, get_reponse_body, request, str_is_url, validate_url, HeaderValue, Method,
};
use crate::core::template::Template;
use crate::utils::errors::other_error;
use crate::utils::string::str_to_bool;
use crate::{
    cli::output::messages::error::TEMPLATE_ALREADY_EXISTS,
    paintln,
    utils::errors::{already_exists_error, invalid_data_error, invalid_input_error, std_error},
};
use std::{io::Error, str, time::Instant};

pub async fn run(command: Command) -> Result<(), Error> {
    repo::create()?;

    if command.args.len() < 1 {
        return Err(invalid_input_error("The template url must be specified."));
    }

    let start = Instant::now(); // start timing process

    let url = if str_is_url(&command.args[0]) {
        validate_url(&command.args[0])?.to_string()
    } else {
        let template_url_path = command.args[0].clone();
        let url = parse_to_raw_url(template_url_path)?;
        validate_url(&url)?.to_string()
    };

    let namespace_name = command.args[0].split("/").collect::<Vec<&str>>()[0];
    let namespace = get_namespace(namespace_name)?;

    let key = if namespace.requires_authorization && command.has_option("key") {
        Ok(command.get_opt_by_name("key").unwrap().value.clone())
    } else {
        Err(invalid_input_error(
            "This remote repo requires authorization key. Add --key=<key> option.",
        ))
    }?;

    paintln!("{gray}", "[getting template]");
    let mut req = build_request(&url, Method::GET, None);

    req.headers_mut().insert(
        "authorization",
        HeaderValue::from_str(&key).expect("Error when set headers."),
    );

    let mut res = request(req).await?;
    let res_body = get_reponse_body(&mut res).await;

    if res.headers().contains_key("message") {
        let is_error = if res.headers().contains_key("isError") {
            let err = std_error(res.headers().get("isError").unwrap().to_str())?;
            str_to_bool(err)
        } else {
            false
        };

        let msg = std_error(res.headers().get("message").unwrap().to_str())?;

        if is_error {
            return Err(other_error(msg));
        }

        println!("{}\n", msg);
    }

    // check if template data is valid
    if serde_json::from_str::<Template>(&res_body).is_err() {
        return Err(invalid_data_error(
            "The remote repo returns an invalid template.",
        ));
    }

    let template: Template = std_error(serde_json::from_str(&res_body))?;

    //check if a template with the same name already exists in repo
    if repo::template_exists(&template.name) {
        return Err(already_exists_error(TEMPLATE_ALREADY_EXISTS));
    }

    repo::save_template(template.clone())?;
    println!("Template \"{}\" was installed.", template.name);

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
