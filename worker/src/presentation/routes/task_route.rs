use actix_web::web;
use crate::presentation::handlers;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(handlers::task_handler::submit)
        .service(handlers::task_handler::notify);
}