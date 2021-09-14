use crate::utils::errors::std_error;
use std::io::Error;

// pub fn split_by(string: String, sep: &str) -> Vec<String> {
//     string
//         .split(sep)
//         .into_iter()
//         .map(|piece: &str| piece.to_string())
//         .collect()
// }

pub fn decode_base64(b64: String) -> Result<String, Error> {
    std_error(String::from_utf8(std_error(base64::decode(b64))?))
}
