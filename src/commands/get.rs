use crate::core::{
    repository::{Template, TemplateManager},
    requester::{Method, ProtternRequester},
};
use std::io::{Error, ErrorKind};

pub async fn get(args: &[String]) -> Result<(), Error> {
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

    let route = format!("/templates/get/{}", template_name);
    let req = ProtternRequester::build_request(route.as_str(), Method::GET, "".to_string());

    let response = match ProtternRequester::request(req).await {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let template: Template = serde_json::from_str(&response).expect("Error when parsing JSON.");

    if let Err(e) = TemplateManager::save_template(template) {
        return Err(e);
    }

    println!("Template was installed.");

    Ok(())
}
