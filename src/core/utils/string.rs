pub fn split_by(string: &str, sep: &str) -> Vec<String> {
    string
        .split(sep)
        .into_iter()
        .map(|piece: &str| piece.to_string())
        .collect()
}

// pub fn str_to_bool(string: &str) -> bool {
//     string.to_lowercase() == "y" || string.to_lowercase() == "true" || string.to_lowercase() == "yes"
// }
