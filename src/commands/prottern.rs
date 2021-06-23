use crate::{core::repository::template_repository_exists, paint};

/* __   __   __
\ \ /  \ / / 
 \ \ /\ \ /
  \_/  \_/ */

const WELCOME_STRING: &'static str = r#"
 _______________________________________________
|   __    __  __  _    ___  ____   _  _   __    |
|   \ \/\/ / ||_ ||   //   ||  || ||\/|| ||_    |
|    \_/\_/  ||_ ||__ \\__ ||__|| ||  || ||_    |
|                      to                       |
|._  _  _           Prottern      _     _      _|
\_______________________________________________|

"#;

pub fn prottern() {
    print!("{}", WELCOME_STRING);

    if !template_repository_exists() {
        paint!(
            "Type \"{yellow}\" to create a template repository.",
            "prottern init"
        );
    }

    paint!(r#"Type "{yellow}" for more information."#, "prottern help")
}
