use crate::{
    core::{
        repository::{create_repository_if_not_exists, RepositoryConnection},
        template::Template,
    },
    paint, paintln,
};
use std::io::Error;

pub fn templates() -> Result<(), Error> {
    create_repository_if_not_exists()?;
    let repository = RepositoryConnection::new();

    if repository.is_empty() {
        println!("Repository is empty.");
        return Ok(());
    }

    let local_templates = repository.get_local_templates();
    let remote_templates = repository.get_remote_templates();

    paintln!("{yellow} Local Templates", ">>");
    print_template_list(local_templates);
    print!("\n");
    paintln!("{yellow} Remote Templates", ">>");
    print_template_list(remote_templates);

    Ok(())
}

fn print_template_list(list: Vec<Template>) {
    for temp in list.iter() {
        paint!("   {gray} ", "|");
        println!("{}", temp.name);
    }
}
