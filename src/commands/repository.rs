use crate::core::repository::{create_repository_if_not_exists, RepositoryConnection};
use crate::core::template::TempMetadata;
use std::io::Error;
use tabled::{Disable, Style, Table};

pub fn repository() -> Result<(), Error> {
    create_repository_if_not_exists()?;
    let repository = RepositoryConnection::new();

    if repository.is_empty() {
        println!("Repository is empty.");
        return Ok(());
    }

    let temps_metadata: Vec<TempMetadata> = {
        let local_templates = repository.get_local_templates();
        let remote_templates = repository.get_remote_templates();
        let templates = [local_templates, remote_templates].concat();
        templates.into_iter().map(|temp| temp.metadata).collect()
    };
    
    let template_tb = Table::new(temps_metadata)
        .with(Disable::Column(4..))
        .with(Style::pseudo());

    println!("{}", template_tb);
    println!("Total templates: {}", repository.total_templates());

    Ok(())
}
