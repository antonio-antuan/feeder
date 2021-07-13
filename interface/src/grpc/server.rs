use super::pb::records::records_service_server::RecordsServiceServer;
use super::pb::sources::sources_service_server::SourcesServiceServer;
use super::pb::users::users_service_server::UsersServiceServer;
use super::records::Service as RecordsService;
use super::sources::Service as SourcesService;
use super::users::Service as UsersService;
use crate::init::App;
use crate::settings::SETTINGS;
use tonic::transport::Server;

pub async fn run_server(app: App) -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = app.storage().pool();

    Server::builder()
        .add_service(UsersServiceServer::with_interceptor(
            UsersService::new(db_pool.clone()),
            logging_interceptor,
        ))
        .add_service(RecordsServiceServer::with_interceptor(
            RecordsService::new(db_pool.clone()),
            logging_interceptor,
        ))
        .add_service(SourcesServiceServer::with_interceptor(
            SourcesService::new(db_pool.clone(), app),
            logging_interceptor,
        ))
        .serve(format!("{}:{}", SETTINGS.server.host, SETTINGS.server.port).parse()?)
        .await?;

    Ok(())
}

fn logging_interceptor(request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    log::info!("request: {:?}", request);
    Ok(request)
}
