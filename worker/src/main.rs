use crate::infrastructure::web;

mod application;
mod domain;
mod infrastructure;
mod presentation;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    web::run().await
}
