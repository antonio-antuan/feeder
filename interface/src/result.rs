use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use tokio_diesel::AsyncError;

#[derive(Debug, PartialEq, Display)]
#[allow(dead_code)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    PoolError(String),
    Unauthorized(String),
}

impl From<AsyncError> for Error {
    fn from(error: AsyncError) -> Error {
        match error {
            AsyncError::Checkout(e) => Error::PoolError(e.to_string()),
            AsyncError::Error(e) => Error::from(e),
        }
    }
}

impl From<DBError> for Error {
    fn from(error: DBError) -> Error {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match &error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::BadRequest(message);
                };
                log::error!("{:?}", error);
                Error::InternalServerError("Unknown database error".into())
            }
            DBError::NotFound => Error::NotFound("object not found".into()),
            _ => {
                log::error!("{}", error);
                Error::InternalServerError("Unknown database error".into())
            }
        }
    }
}

impl From<feeder::result::Error> for Error {
    fn from(error: feeder::result::Error) -> Self {
        Error::InternalServerError(error.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
