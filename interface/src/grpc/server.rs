use super::proto::records::records_service_server::RecordsServiceServer;
use super::proto::users::users_service_server::UsersServiceServer;
use super::records::Service as RecordsService;
use super::users::Service as UsersService;
use crate::db::Pool;
use tonic::transport::Server;

pub async fn run_server(grpc_addr: &str, db_pool: Pool) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("starting grpc server");

    Server::builder()
        .add_service(UsersServiceServer::new(UsersService::new(db_pool.clone())))
        .add_service(RecordsServiceServer::new(RecordsService::new(db_pool)))
        .serve(grpc_addr.parse()?)
        .await?;

    Ok(())
}
