use actix_web::web;
use crate::presentation::handlers;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .service(handlers::probe_handler::health)
            .service(handlers::probe_handler::ready)
    );
}