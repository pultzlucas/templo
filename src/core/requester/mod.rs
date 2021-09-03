mod requester;

pub use hyper::http::HeaderValue;
pub use hyper::Method;
pub use requester::{build_request, request};