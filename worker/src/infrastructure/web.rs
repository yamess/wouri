use crate::presentation::routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use crate::shared::app_state::AppState;

pub async fn run() -> std::io::Result<()> {
    log::info!("Server is running on http://0.0.0.0:8080");
    let app_state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(
                web::scope("/api/v1")
                    .configure(routes::task_route::routes)
                    .configure(routes::probe_route::routes)
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
