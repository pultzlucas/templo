use crate::{
    core::{
        template::Template,
        requester::{Method, ProtternRequester},
    },
    paintln,
};

use std::io::Error;

pub async fn discover() -> Result<(), Error> {
    let templates = {
        let req = ProtternRequester::build_request("/templates", Method::GET, "".to_owned());

        paintln!("{gray}", "[Searching Templates]");

        let response = ProtternRequester::request(req).await?;
        serde_json::from_str(&response).unwrap()
    };

    show_templates(templates);

    Ok(())
}

fn show_templates(templates: Vec<Template>) {
    for template in templates.into_iter() {
        println!("- {}", template.name);
    }
}
