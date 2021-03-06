use crate::db::models::User;
use crate::db::{queries::users as users_queries, Pool};
use crate::result::Result;
use crate::rest::auth;
use actix_web::web::{Data, Json};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginPasswordRequest {
    pub login: String,
    pub password: String,
}

pub async fn login(request: Json<LoginPasswordRequest>, db_pool: Data<Pool>) -> Result<Json<User>> {
    Ok(Json(
        auth::login_user(&db_pool, request.login.clone(), request.password.clone()).await?,
    ))
}

pub async fn register(
    request: Json<LoginPasswordRequest>,
    db_pool: Data<Pool>,
) -> Result<Json<User>> {
    let password = auth::hash(request.password.as_str());
    Ok(Json(
        users_queries::create_user(&db_pool, request.login.clone(), password).await?,
    ))
}
