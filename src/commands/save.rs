pub fn save(args: &[String]) -> Result<String, String> {
    if args.len() < 1 {
        return Err("Folder name must be specified.".to_string())
    }

    if args.len() < 2 {
        return Err("Template name must be specified.".to_string())
    }

    let folder = &args[0];
    let template_name = &args[1];

    Ok(format!("Folder: {}\nTemplate Name: {}", folder, template_name))
}
