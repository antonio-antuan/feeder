use crate::result::Error;

mod proto;
mod records;
pub mod server;
mod users;

impl From<Error> for tonic::Status {
    fn from(err: Error) -> Self {
        match err {
            Error::BadRequest(m) => tonic::Status::internal(m),
            Error::InternalServerError(m) => tonic::Status::internal(m),
            Error::NotFound(m) => tonic::Status::not_found(m),
            Error::PoolError(m) => tonic::Status::internal(m),
            Error::Unauthorized(m) => tonic::Status::unauthenticated(m),
        }
    }
}

async fn auth_user(
    db_pool: &crate::db::Pool,
    md: &tonic::metadata::MetadataMap,
) -> crate::result::Result<crate::db::models::User, tonic::Status> {
    let unauthorized: tonic::Status = tonic::Status::unauthenticated("unauthorized");
    match md.get("token") {
        None => Err(unauthorized),
        Some(token) => Ok(crate::auth::auth_user(
            db_pool,
            token
                .to_str()
                .map_err(|_e| tonic::Status::internal("cannot decode string"))?
                .to_string(),
        )
        .await?
        .ok_or(unauthorized)?),
    }
}
