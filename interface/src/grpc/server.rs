use super::proto::users::users_service_server::UsersServiceServer;
use super::users::Service;
use crate::db::Pool;
use tonic::transport::Server;

pub async fn run_server(grpc_addr: &str, db_pool: Pool) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting Solar System info server");

    let svc = UsersServiceServer::new(Service::new(db_pool));

    Server::builder()
        .add_service(svc)
        .serve(grpc_addr.parse()?)
        .await?;

    Ok(())
}
