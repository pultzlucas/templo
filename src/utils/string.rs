pub fn split_by(string: String, sep: &str) -> Vec<String> {
    string
        .split(sep)
        .into_iter()
        .map(|piece: &str| piece.to_string())
        .collect()
}

pub fn decode_base64(b64: String) -> String {
    String::from_utf8(base64::decode(b64).unwrap()).unwrap()
}