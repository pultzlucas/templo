use crate::paintln;
use crate::cli::output::clear_console;

const WELCOME_STRING: &'static str = r#" 
 _______________________________________________
|   __    __  __  _    ___  ____   _  _   __    |
|   \ \/\/ / ||_ ||   //   ||  || ||\/|| ||_    |
|    \_/\_/  ||_ ||__ \\__ ||__|| ||  || ||_    |
|                      to                       |
|._  _  _           Prottern      _     _      _|
\_______________________________________________|

"#;

pub fn run() {
    clear_console();
    print!("{}", WELCOME_STRING);
    paintln!(r#"Type "{yellow}" for more information."#, "prottern help");
}
