use super::pb::users;
use crate::auth;
use crate::db::queries::{folders as folders_queries, users as users_queries};
use crate::db::Pool;
use crate::grpc::pb::users::{
    AddFolderRequest, AddFolderResponse, RemoveFolderRequest, RemoveFolderResponse,
};
use std::convert::TryInto;
use tonic::{Request, Response, Status};

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

    async fn get_folders(
        &self,
        request: tonic::Request<users::GetFoldersRequest>,
    ) -> Result<tonic::Response<users::GetFoldersResponse>, Status> {
        let user = super::auth_user(&self.db_pool, request.metadata()).await?;
        let folders = folders_queries::get_user_folders(&self.db_pool, user.id).await?;
        Ok(tonic::Response::new(users::GetFoldersResponse {
            folders: folders.into_iter().map(From::from).collect(),
        }))
    }

    async fn add_folder(
        &self,
        request: Request<AddFolderRequest>,
    ) -> Result<Response<AddFolderResponse>, Status> {
        let user = super::auth_user(&self.db_pool, request.metadata()).await?;
        let message: AddFolderRequest = request.into_inner();
        let folder_id = folders_queries::add_user_folder(
            &self.db_pool,
            user.id,
            message.name,
            match &message.parent_folder_id {
                0 => None,
                _ => Some(message.parent_folder_id),
            },
        )
        .await?;
        Ok(tonic::Response::new(users::AddFolderResponse {
            id: folder_id,
        }))
    }

    async fn remove_folder(
        &self,
        request: Request<RemoveFolderRequest>,
    ) -> Result<Response<RemoveFolderResponse>, Status> {
        let user = super::auth_user(&self.db_pool, request.metadata()).await?;
        let message: RemoveFolderRequest = request.into_inner();
        folders_queries::remove_user_folder(&self.db_pool, user.id, message.id).await?;
        Ok(tonic::Response::new(users::RemoveFolderResponse {}))
    }
}
