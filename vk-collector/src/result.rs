use std::fmt;
use crate::types::{VkError, VkResponse};
pub type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Debug, Clone)]
pub enum Error {
    VkError(VkError),
    RequestTimeout,
    Internal(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::RequestTimeout
        } else {
            Error::Internal(err.to_string())
        }
    }
}
