use crate::{paintln, paint, paint_string};

pub fn run() {
    let welcome_msg = paint_string!("{yellow}", "WELCOME to TEMPLO");
    let welcome_string = vec![
        "┌────────────────────────────────────────────────┐",
&format!("│               {}                │", welcome_msg),
        "├───┬───┬───┬───┬───┐       ┌───┬───┬───┬───┬────┤",
        "│   │   │   │   │   │       │   │   │   │   │    │",
        "│   │   │   │   │   │       │   │   │   │   │    │",
        "│   │   │   │   │   │       │   │   │   │   │    │",
        "│   │   │   │   │   │       │   │   │   │   │    │",
        "├───┘   └───┘   └───┘       └───┘   └───┘   └────┤",
        "└────────────────────────────────────────────────┘",
        ""
    ].join("\n");
    
    print!("{}", welcome_string);
    paintln!(r#"Type "{yellow}" for more information."#, "tp --help");
}
