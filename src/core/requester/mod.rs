mod requester;

pub use hyper::http::HeaderValue;
pub use hyper::Method;
pub use requester::{build_request, request};

pub const AUTHENTICATOR_URL: &'static str = "https://prottern-authenticator.herokuapp.com";