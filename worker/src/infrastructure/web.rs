use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use crate::presentation::routes;

pub async fn run() -> std::io::Result<()> {
    log::info!("Server is running on http://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::probe_route::routes)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}