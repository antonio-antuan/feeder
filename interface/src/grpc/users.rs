use super::pb::users;
use crate::auth;
use crate::db::queries::{folders as folders_queries, users as users_queries};
use crate::db::Pool;
use std::convert::TryInto;

const BASE_FOLDER: &str = "BASE";

#[derive(Clone)]
pub struct Service {
    db_pool: Pool,
}

impl Service {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }
}

#[tonic::async_trait]
impl users::users_service_server::UsersService for Service {
    async fn login(
        &self,
        request: tonic::Request<users::LoginRequest>,
    ) -> Result<tonic::Response<users::LoginResponse>, tonic::Status> {
        let message: users::LoginRequest = request.into_inner();

        let user =
            auth::login_user(&self.db_pool, &message.login, message.password.as_str()).await?;
        Ok(tonic::Response::new(users::LoginResponse {
            user: Some(user.try_into()?),
        }))
    }

    async fn register(
        &self,
        request: tonic::Request<users::RegisterRequest>,
    ) -> Result<tonic::Response<users::RegisterResponse>, tonic::Status> {
        let message: users::RegisterRequest = request.into_inner();
        let password = auth::hash(message.password.as_str());
        let user =
            users_queries::create_user(&self.db_pool, message.login.clone(), password).await?;
        log::info!("{:?}", user);
        folders_queries::add_user_folder(&self.db_pool, user.id, BASE_FOLDER.to_string(), None)
            .await?;
        Ok(tonic::Response::new(users::RegisterResponse {
            user: Some(user.try_into()?),
        }))
    }
}
