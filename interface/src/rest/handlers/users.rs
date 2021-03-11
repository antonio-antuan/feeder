use crate::db::models::User;
use crate::db::{queries::users as users_queries, Pool};
use crate::rest::auth;
use crate::result::Result;
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
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


#[cfg(test)]
mod tests {
    use crate::db;
    use crate::settings;

    use actix_web::{test, web, App};
    use super::{register, LoginPasswordRequest};
    use actix_web::http::StatusCode;
    use actix_web::body::{Body, ResponseBody};
    use crate::db::models::User;

    #[actix_rt::test]
    async fn test() {
        let pool = db::init_pool(settings::SETTINGS.database.url.as_str());
        let mut app = test::init_service(
            App::new()
                .app_data(pool)
                .route("/", web::post().to(register)),
        )
        .await;
        let empty_req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&mut app, empty_req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let req_data = LoginPasswordRequest{ login: "test".to_string(), password: "test".to_string() };
        let valid_req = test::TestRequest::post().uri("/")
            .set_json(&req_data)
            .to_request();
        let resp = test::call_service(&mut app, valid_req).await;
        assert_eq!(resp.status(), StatusCode::OK, "{:?}", resp);
    }
}