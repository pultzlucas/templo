use crate::core::repository::delete_template;

pub fn delete(args: &[String]) -> Result<&str, String> {
    if args.len() < 1  {
        return Err("Template name must be specified.".to_string())
    }

    if let Err(e) = delete_template(&args[0]) {
        return Err(e)
    } 

    Ok("Template was deleted.")
}