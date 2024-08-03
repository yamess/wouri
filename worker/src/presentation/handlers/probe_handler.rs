use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn health() -> impl Responder {
    log::debug!("Health check");
    HttpResponse::Ok().body("Ok")
}

#[get("/ready")]
pub async fn ready() -> impl Responder {
    log::debug!("Ready check");
    HttpResponse::Ok().body("Ok")
}
