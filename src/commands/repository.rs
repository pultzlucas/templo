use crate::cli::input::args::Args;
use crate::core::fs::paths::get_repo_path;
use crate::utils::path::pathbuf_to_string;
use crate::core::repo;
use crate::core::template::{Template, TemplateDisplayInfo};
use std::io::Error;
use tabled::{Disable, Style, Table};

pub fn run(args: Args) -> Result<(), Error> {
    repo::create()?;

    if args.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_repo_path().unwrap()));
        return Ok(());
    }

    if repo::is_empty() {
        println!("Repository is empty.");
        return Ok(());
    }

    let templates = repo::get_templates()?;
    let templates_display: Vec<TemplateDisplayInfo> = templates
        .into_iter()
        .map(|temp: Template| temp.fmt())
        .collect();

    let template_tb = Table::new(templates_display)
        .with(Disable::Column(4..))
        .with(Style::pseudo());

    println!("{}", template_tb);
    println!("Total templates: {}", repo::total_templates());

    Ok(())
}
