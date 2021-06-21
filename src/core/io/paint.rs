#[macro_export]
macro_rules! paint {
    ($text:expr, $($string:expr),*) => {
        //\{(\w+|:)*}
        {
            use regex::{Captures, Regex};
            let regex = Regex::new(r"\{(\w+|:)*}").unwrap();
            let mut strings = Vec::new();
            $(strings.push($string);)*

            let _paint_string = |string: &str, color_name: &str| {
                let color_id = match color_name {
                    "red" => 31,
                    "green" => 32,
                    "yellow" => 33,
                    _ => panic!("Invalid color name in paint macro.")
                };
                format!("\x1b[1;{}m{}\x1b[0;0m", color_id, string)
            };


            let output = regex.replace_all($text, |caps: &Captures|{
                if strings.len() == 0 {
                    panic!("Invalid numbers of string parameters in paint macro.");
                }
                let painted_string = _paint_string(strings[0], &caps[1]);
                strings.remove(0);
                painted_string
            });
            if strings.len() > 0 {
                panic!("Invalid numbers of placeholders in paint macro.");
            }

            println!("{}", output);
        }
    };
}
