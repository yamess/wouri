#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse environment variable {0}")]
    EnvVarError(String),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Rabbit error: {0}")]
    RabbitError(#[from] lapin::Error),
    #[error("Actix error: {0}")]
    ActixError(#[from] actix_web::Error),
    #[error("Deadpool error: {0}")]
    DeadpoolError(#[from] deadpool_redis::PoolError),
}

pub type Result<T> = std::result::Result<T, Error>;
