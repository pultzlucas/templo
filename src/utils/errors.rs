use std::io::{Error, ErrorKind};
use std::string::ToString;

// Convert others error types to std::io::Error type
pub fn std_error<T, E: ToString>(result: Result<T, E>) -> Result<T, Error>{
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

pub fn invalid_input_error(msg: &str) -> Error {
    return Error::new(ErrorKind::InvalidInput, msg);
}

pub fn not_found_error(msg: &str) -> Error {
    return Error::new(ErrorKind::NotFound, msg);
}

// pub fn permission_denied_error(msg: &str) -> Error {
//     return Error::new(ErrorKind::NotFound, msg);
// }

pub fn repo_connection_error(msg: &str) -> Error {
    return Error::new(ErrorKind::ConnectionAborted, msg);
}

pub fn other_error(msg: &str) -> Error {
    return Error::new(ErrorKind::NotFound, msg);
}

pub fn already_exists_error(msg: &str) -> Error {
    return Error::new(ErrorKind::NotFound, msg);
}

pub fn invalid_data_error(msg: &str) -> Error {
    return Error::new(ErrorKind::InvalidData, msg);
}