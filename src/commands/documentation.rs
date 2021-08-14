use crate::paintln;
use open;
use std::io::Error;
use std::time::Instant;

const DOCS_URL: &'static str = "https://github.com/Lucas-Lopes-Pultz/prottern-docs";

pub fn run() -> Result<(), Error> {
    let start = Instant::now(); // start timing process
    paintln!("{gray}", "[Opening browser]");

    //open browser
    open::that(DOCS_URL)?;

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));
    println!("Look at your browser!");
    Ok(())
}
