use crate::cli::input::command::Command;
use crate::utils::path::pathbuf_to_string;
use crate::core::repos;
use crate::core::template::{Template, TemplateDisplayInfo};
use std::io::Error;
use tabled::{Disable, Style, Table};

pub fn run(command: Command) -> Result<(), Error> {
    let flags = vec!["--local", "--total"];
    check_flags(&command.flags, flags)?;

    if command.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_repo_path().unwrap()));
        return Ok(());
    }

    if command.has_flag("--total") {
        println!("{}", repos::total_templates());
        return Ok(());
    }

    if repos::is_empty() {
        println!("Repository is empty.");
        return Ok(());
    }

    let templates = repos::get_templates()?;
    let templates_display: Vec<TemplateDisplayInfo> = templates
        .into_iter()
        .map(|temp: Template| temp.fmt())
        .collect();

    let template_tb = Table::new(templates_display)
        .with(Disable::Column(4..))
        .with(Style::pseudo());

    print!("{}", template_tb);

    Ok(())
}
