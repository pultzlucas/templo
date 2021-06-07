use crate::core::repository::get_template;

pub fn describe(args: &[String]) -> Result<&str, String> {
    if args.len() < 1 {
        return Err("Template name must be specified.".to_string());
    }

    let template_name = &args[0];
    let template = match get_template(template_name) {
        Ok(t) => t,
        Err(e) => return Err(e)
    };
    
    let paths_splitted: Vec<&str> = template.paths.split(";").collect();

    println!("--- name ---");
    println!("{}", template.name);

    println!("--- paths ---");
    paths_splitted.iter().for_each(|path| {
        println!("{}", path);
    });

    Ok("")
}
