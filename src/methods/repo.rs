use crate::cli::input::command::Command;
use crate::core::path::get_repo_path;
use crate::methods::check_flags;
use crate::utils::path::pathbuf_to_string;
use crate::core::repo;
use crate::core::template::{Template, TemplateDisplayInfo};
use std::io::Error;
use tabled::{Disable, Style, Table};

pub fn run(command: Command) -> Result<(), Error> {
    repo::create()?;

    let flags = vec!["--local", "--total"];
    check_flags(&command.flags, flags)?;

    if command.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_repo_path().unwrap()));
        return Ok(());
    }

    if command.has_flag("--total") {
        println!("{}", repo::total_templates());
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

    print!("{}", template_tb);

    Ok(())
}
