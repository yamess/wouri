use crate::presentation::handlers;
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(handlers::probe_handler::health)
        .service(handlers::probe_handler::ready);
}
