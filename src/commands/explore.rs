use crate::{core::repository::remote, paintln};
use std::io::Error;
use std::time::Instant;
use tabled::{Disable, Style, Table};

pub async fn explore() -> Result<(), Error> {
    paintln!("{gray}", "[Searching Templates]");

    let start = Instant::now(); // start timing process
    let templates = remote::get_all_templates().await?;
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    let templates_tb = Table::new(templates)
        .with(Disable::Column(4..))
        .with(Disable::Column(3..4))
        .with(Style::pseudo());

    print!("{}", templates_tb);

    Ok(())
}
