use std::fmt::Display;
use std::time::Duration;

pub mod store;
pub use store::*;
pub mod descriptor;
pub use descriptor::*;
pub mod item;
pub use item::*;

#[derive(Debug)]
pub struct Error {
    _http_status_code: Option<usize>,
    _retry_after: Option<Duration>,
}

impl Error {
    pub fn new() -> Self {
        Self {
            _http_status_code: None,
            _retry_after: None,
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}