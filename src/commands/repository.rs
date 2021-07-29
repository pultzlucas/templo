use crate::core::{
    repository::{create_repository_if_not_exists, RepositoryConnection},
};
use std::io::Error;
use tabled::{Style, Table, Disable};

pub fn repository() -> Result<(), Error> {
    create_repository_if_not_exists()?;
    let repository = RepositoryConnection::new();

    if repository.is_empty() {
        println!("Repository is empty.");
        return Ok(());
    }

    let local_templates = repository.get_local_templates();
    let remote_templates = repository.get_remote_templates();
    let all_templates = [local_templates, remote_templates].concat();

    let template_tb = Table::new(all_templates)
        .with(Style::pseudo())
        .with(Disable::Column(4..));

    print!("{}", template_tb);

    Ok(())
}
