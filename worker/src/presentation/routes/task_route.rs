use crate::presentation::handlers;
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(handlers::task_handler::submit)
        .service(handlers::task_handler::notify);
}
