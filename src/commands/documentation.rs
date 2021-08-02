use std::io::Error;
use crate::paintln;
use open;

const DOCS_URL: &'static str = "https://github.com/Lucas-Lopes-Pultz/prottern-docs";

pub fn documentation() -> Result<(), Error> {
    paintln!("{gray}", "[Opening browser]");
    open::that(DOCS_URL)?;
    println!("Look at your browser!");
    Ok(())
}