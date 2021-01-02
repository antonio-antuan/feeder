use rust_tdlib::errors::RTDError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Common(String),
    UpdateNotSupported(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<RTDError> for Error {
    fn from(err: RTDError) -> Self {
        Self::Common(err.to_string())
    }
}
