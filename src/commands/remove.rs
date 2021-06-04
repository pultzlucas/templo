use crate::core::repository::remove_template;

pub fn remove(args: &[String]) -> Result<&str, String> {
    if args.len() < 1  {
        return Err("Template name must be specified.".to_string())
    }

    if let Err(e) = remove_template(&args[0]) {
        return Err(e)
    } 

    Ok("Template was removed.")
}