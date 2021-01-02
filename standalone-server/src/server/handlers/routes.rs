use super::records::{get_records, get_records_for_preview, mark_record};
use super::sources::{get_list, search, subscribe, unsubscribe};
use super::users::{login, register};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/users")
                    .route("/login", web::post().to(login))
                    .route("/register", web::post().to(register)),
            )
            .service(
                web::scope("/records")
                    .route("/", web::get().to(get_records))
                    .route("/preview", web::get().to(get_records_for_preview))
                    .route("/{record_id}", web::post().to(mark_record)),
            )
            .service(
                web::scope("/sources")
                    .route("/", web::get().to(get_list))
                    .route("/search", web::get().to(search))
                    .route("/{source_id}", web::delete().to(unsubscribe))
                    .route("/{source_id}", web::put().to(subscribe)),
            ),
    );
}
