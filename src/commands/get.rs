use crate::{
    core::{
        repository::{Template, TemplateManager},
        requester::{Method, ProtternRequester},
    },
    init,
};
use std::io::{Error, ErrorKind};

pub async fn get(args: &[String]) -> Result<(), Error> {
    init()?;

    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    let template_name = &args[0];

    if TemplateManager::template_exists(&template_name) {
        let err = Error::new(
            ErrorKind::AlreadyExists,
            format!(
                "Template \"{}\" already exists on repository.",
                template_name
            ),
        );
        return Err(err);
    }

    let template: Template = {
        let response = {
            let req = {
                let route = format!("/templates/get/{}", template_name);
                ProtternRequester::build_request(route.as_str(), Method::GET, "".to_string())
            };
            ProtternRequester::request(req).await?
        };
        serde_json::from_str(&response).expect("Error when parsing JSON.")
    };

    if let Err(e) = TemplateManager::save_template(template) {
        return Err(e);
    }

    println!("Template was installed.");

    Ok(())
}
