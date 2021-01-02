use atom_syndication::Error as AtomError;
use rss::Error as RSSError;
use std::fmt;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub enum Error {
    NoFeed,
    SourceNotSupported,
    ScrapeTimeout,
    InvalidUrl(url::ParseError),
    RequestError,
    // can't decode raw content
    DecodeError,
    // can't parse particular format
    ParseError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::ScrapeTimeout
        } else if err.is_decode() {
            Error::DecodeError
        } else {
            Error::RequestError
        }
    }
}

impl From<RSSError> for Error {
    fn from(_err: RSSError) -> Self {
        Self::ParseError
    }
}

impl From<AtomError> for Error {
    fn from(_err: AtomError) -> Self {
        Self::ParseError
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::InvalidUrl(err)
    }
}
