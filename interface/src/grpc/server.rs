use super::records::Service as RecordsService;
use super::sources::Service as SourcesService;
use super::users::Service as UsersService;
use crate::init::App;
use crate::proto::records::records_service_server::RecordsServiceServer;
use crate::proto::sources::sources_service_server::SourcesServiceServer;
use crate::proto::users::users_service_server::UsersServiceServer;
use crate::settings::SETTINGS;
use tonic::transport::Server;

pub async fn run_server(app: App) -> Result<(), Box<dyn std::error::Error>> {
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
        .serve(format!("{}:{}", SETTINGS.server.host, SETTINGS.server.port).parse()?)
        .await?;

    Ok(())
}
