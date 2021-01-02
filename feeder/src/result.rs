use std::fmt;

#[derive(Debug)]
pub enum Error {
    DbError(String),
    HttpCollectorError(http_collector::result::Error),
    TgCollectorError(tg_collector::result::Error),
    UpdateNotSupported(String),
    SourceKindConflict(String),
    SourceNotFound,
    SourceCreationError,
    IOError(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<http_collector::result::Error> for Error {
    fn from(err: http_collector::result::Error) -> Self {
        Self::HttpCollectorError(err)
    }
}

impl From<tg_collector::result::Error> for Error {
    fn from(err: tg_collector::result::Error) -> Self {
        Self::TgCollectorError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
