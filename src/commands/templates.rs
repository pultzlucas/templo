use crate::{
    core::repository::{Template, TemplateManager, TemplateType},
    init,
};
use std::io::Error;

pub fn templates() -> Result<(), Error> {
    init()?;
    if let Some(templates) = TemplateManager::get_all_templates() {
        let local_templates: Vec<&Template> = templates
            .iter()
            .filter(|temp| temp.template_type == TemplateType::Local)
            .collect();

        let remote_templates: Vec<&Template> = templates
            .iter()
            .filter(|temp| temp.template_type == TemplateType::Remote)
            .collect();
            //\033[48:5:208:0mLocal Templates%s\033[m
        println!("#[Local Templates]");
        print_template_list(local_templates);
        print!("\n");
        println!("#[Remote Templates]");
        print_template_list(remote_templates);

    } else {
        println!("Repository is empty.");
    }

    Ok(())
}

fn print_template_list(list: Vec<&Template>) {
    for temp in list.iter() {
        println!("- {}", temp.name);
    }
}
