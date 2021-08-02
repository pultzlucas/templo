use crate::{
    core::{
        requester::{Method, ProtternRequester},
        template::Template,
    },
    paintln,
};
use std::io::Error;
use std::time::Instant;
use tabled::{Disable, Style, Table};

pub async fn explore() -> Result<(), Error> {
    // get templates
    let templates: Vec<Template> = {
        let requester = ProtternRequester::new();
        let req = requester.build_request("/templates", Method::GET, "".to_owned());
        paintln!("{gray}", "[Searching Templates]");
        let start = Instant::now(); // start timing process
        let response = requester.request(req).await?;
        let templates: Vec<Template> = serde_json::from_str(&response).unwrap();
        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));
        templates
    };

    let templates_tb = Table::new(templates)
        .with(Disable::Column(4..))
        .with(Disable::Column(3..4))
        .with(Style::pseudo());

    print!("{}", templates_tb);

    Ok(())
}
