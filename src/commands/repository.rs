use crate::core::repository::local;
use crate::core::template::{Template, TemplateDisplayInfo};
use std::io::Error;
use tabled::{Disable, Style, Table};

pub fn run() -> Result<(), Error> {
    local::create()?;

    if local::is_empty() {
        println!("Repository is empty.");
        return Ok(());
    }

    let templates = local::get_templates();
    let templates_display: Vec<TemplateDisplayInfo> = templates
        .into_iter()
        .map(|temp: Template| temp.fmt())
        .collect();

    let template_tb = Table::new(templates_display)
        .with(Disable::Column(4..))
        .with(Style::pseudo());

    println!("{}", template_tb);
    println!("Total templates: {}", local::total_templates());

    Ok(())
}
