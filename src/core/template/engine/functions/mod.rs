use crate::core::utils::string::split_by;

#[allow(dead_code)]
pub fn upper(v: &str) -> String {
    v.to_uppercase()
}

#[allow(dead_code)]
pub fn lower(value: &str) -> String {
    value.to_lowercase()
}

#[allow(dead_code)]
pub fn join(value: &str, sep: Option<&str>) -> String {
    let sep = if sep.is_none() { Some(" ") } else { sep };
    if let Some(sep) = sep {
        let array = split_by(value, sep);
        array.join("")
    } else {
        panic!("Something went wrong in engine join function");
    }
}

#[allow(dead_code)]
pub fn upper_first(value: &str) -> String {
    let first_char: Vec<String> = value
        .chars()
        .enumerate()
        .into_iter()
        .map(|(i, ch)| {
            if i == 0 {
                return ch.to_uppercase().to_string();
            }
            ch.to_string()
        })
        .collect();

    first_char.join("")
}
