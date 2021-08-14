use crate::core::template::{Template, TemplateDisplayInfo};
use crate::{core::repository::remote, paintln};
use std::io::Error;
use std::time::Instant;
use tabled::{Disable, Style, Table};

pub async fn run() -> Result<(), Error> {
    paintln!("{gray}", "[Searching Templates]");

    let start = Instant::now(); // start timing process
    let templates = remote::get_all_templates().await?;

    if templates.len() == 0 {
        println!("No there public templates.");
        return Ok(());
    }

    let templates_display: Vec<TemplateDisplayInfo> = templates
        .into_iter()
        .map(|temp: Template| temp.fmt())
        .collect();

    let templates_tb = Table::new(templates_display)
        .with(Disable::Column(4..))
        .with(Disable::Column(3..4))
        .with(Style::pseudo());

    print!("{}", templates_tb);
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
