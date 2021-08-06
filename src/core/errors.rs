use std::io::{Error, ErrorKind};

pub fn invalid_input_error(msg: &str) -> Error {
    return Error::new(ErrorKind::InvalidInput, msg);
}