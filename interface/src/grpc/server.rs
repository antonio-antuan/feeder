use super::proto::records::records_service_server::RecordsServiceServer;
use super::proto::sources::sources_service_server::SourcesServiceServer;
use super::proto::users::users_service_server::UsersServiceServer;
use super::records::Service as RecordsService;
use super::sources::Service as SourcesService;
use super::users::Service as UsersService;
use crate::init::App;
use tonic::transport::Server;

pub async fn run_server(grpc_addr: &str, app: App) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("starting grpc server");
    let db_pool = app.storage().pool();

    Server::builder()
        .add_service(UsersServiceServer::new(UsersService::new(db_pool.clone())))
        .add_service(RecordsServiceServer::new(RecordsService::new(
            db_pool.clone(),
        )))
        .add_service(SourcesServiceServer::new(SourcesService::new(
            db_pool.clone(),
            app,
        )))
        .serve(grpc_addr.parse()?)
        .await?;

    Ok(())
}
