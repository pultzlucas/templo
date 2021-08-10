use crate::{
    core::{
        requester::{Method, build_request, request},
        template::{Template, TempMetadata},
    },
    paintln,
};
use std::io::Error;
use std::time::Instant;
use tabled::{Disable, Style, Table};

pub async fn explore() -> Result<(), Error> {
    // get templates
    let templates: Vec<TempMetadata> = {
        let req = build_request("/templates", Method::GET, "".to_owned());
        paintln!("{gray}", "[Searching Templates]");
        let start = Instant::now(); // start timing process
        let response = request(req).await?;
        let templates: Vec<Template> = serde_json::from_str(&response).unwrap();
        let temp_metadata: Vec<TempMetadata> = templates.into_iter().map(|temp| temp.metadata).collect();
        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));
        temp_metadata
    };

    let templates_tb = Table::new(templates)
        .with(Disable::Column(4..))
        .with(Disable::Column(3..4))
        .with(Style::pseudo());

    print!("{}", templates_tb);

    Ok(())
}
