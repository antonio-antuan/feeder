use crate::result::Error;

mod proto;
pub mod server;
mod users;

impl From<crate::result::Error> for tonic::Status {
    fn from(err: crate::result::Error) -> Self {
        match err {
            Error::BadRequest(m) => tonic::Status::internal(m),
            Error::InternalServerError(m) => tonic::Status::internal(m),
            Error::NotFound(m) => tonic::Status::not_found(m),
            Error::PoolError(m) => tonic::Status::internal(m),
            Error::Unauthorized(m) => tonic::Status::unauthenticated(m),
        }
    }
}
