use crate::cli::input::command::Command;
use crate::core::path::get_repo_path;
use crate::core::repos::Repository;
use crate::core::template::{Template, TemplateDisplayInfo};
use crate::methods::check_flags;
use crate::utils::path::pathbuf_to_string;
use std::io::Error;
use tabled::{Disable, Style, Table};

pub struct Repo;

impl Repo {
    pub fn run(command: Command) -> Result<(), Error> {
        let flags = vec!["--local", "--total"];
        check_flags(&command.flags, flags)?;

        let repo_name = if command.args.len() > 0 {
            command.args[0].clone()
        } else {
            "main".to_string()
        };
        let repo = Repository::connect(repo_name)?;

        if command.has_flag("--local") {
            println!("{}", pathbuf_to_string(get_repo_path(&repo.name).unwrap()));
            return Ok(());
        }

        if command.has_flag("--total") {
            println!("{}", repo.total_templates());
            return Ok(());
        }

        if repo.is_empty() {
            println!("Repository is empty.");
            return Ok(());
        }

        let templates = repo.get_templates()?;
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
}
