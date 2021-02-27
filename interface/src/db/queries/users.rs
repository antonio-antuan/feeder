use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind as DieselDatabaseErrorKind, Error as DieselError};

use crate::db::models::User;
use crate::db::Pool;
use crate::server::result::ApiError;
use crate::schema::users;
use tokio_diesel::*;

pub async fn get_user_by_token(db_pool: &Pool, token: String) -> Result<Option<User>, ApiError> {
    let user = users::table
        .filter(users::token.eq(token))
        .first_async::<User>(db_pool)
        .await;
    match user {
        Ok(user) => Ok(Some(user)),
        Err(AsyncError::Error(diesel::result::Error::NotFound)) => Ok(None),
        Err(err) => Err(err.into()),
    }
}

pub async fn create_user(
    db_pool: &Pool,
    login: String,
    hashed_password: String,
) -> Result<User, ApiError> {
    match insert_into(users::table)
        .values((users::login.eq(login), users::password.eq(hashed_password)))
        .get_result_async::<User>(db_pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(AsyncError::Error(DieselError::DatabaseError(
            DieselDatabaseErrorKind::UniqueViolation,
            _info,
        ))) => Err(ApiError::ValidationError(vec![
            "user already exists".to_string()
        ])),
        Err(err) => Err(err.into()),
    }
}

pub async fn get_user_by_login(db_pool: &Pool, login: String) -> Result<User, ApiError> {
    Ok(users::table
        .filter(users::login.eq(login))
        .first_async::<User>(db_pool)
        .await?)
}
