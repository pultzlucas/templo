use crate::core::{
    repository::Template,
    requester::{Method, ProtternRequester},
};
use std::io::Error;

pub async fn discover() -> Result<(), Error> {
    let req = ProtternRequester::build_request("/templates", Method::GET, "".to_owned());

    match ProtternRequester::request(req).await {
        Err(e) => return Err(e),
        Ok(response) => show_templates(serde_json::from_str(&response).unwrap()),
    }

    Ok(())
}

fn show_templates(templates: Vec<Template>) {
    for template in templates.into_iter() {
        println!("- {}", template.name);
    }
}
