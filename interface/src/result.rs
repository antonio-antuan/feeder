use derive_more::Display;

#[derive(Debug, PartialEq, Display)]
#[allow(dead_code)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    PoolError(String),
    Unauthorized(String),
}

impl From<feeder::result::Error> for Error {
    fn from(error: feeder::result::Error) -> Self {
        Error::InternalServerError(error.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Error {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match &error {
            sqlx::Error::RowNotFound => Error::NotFound("object not found".into()),
            _ => {
                log::error!("{:?}", error);
                Error::InternalServerError("Unknown database error".into())
            }
        }
    }
}
