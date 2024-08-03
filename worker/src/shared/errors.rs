#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse environment variable {0}")]
    EnvVarError(String),
    #[error("Pool error: {0}")]
    PoolError(#[from] r2d2::Error),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Rabbit error: {0}")]
    RabbitError(#[from] lapin::Error),
    #[error("Actix error: {0}")]
    ActixError(#[from] actix_web::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
