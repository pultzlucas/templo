use crate::core::get_template_paths;
use crate::utils::structs::Template;

pub fn save(args: &[String]) -> Result<&str, String> {
    if args.len() < 1 {
        return Err("Folder name must be specified.".to_string())
    }

    if args.len() < 2 {
        return Err("Template name must be specified.".to_string())
    }

    let directory = args[0].clone();
    let template_name = args[1].clone();
    let template_head = Template {
        name: template_name,
        paths: get_template_paths(directory).join(";")
    };

    println!("HEAD: \n{:?}", template_head);

    Ok("saved!")
}
