use redis::RedisError;

pub type Result<T> = std::result::Result<T, Error>;

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
    #[error(transparent)]
    Task(#[from] TaskError),
    #[error(transparent)]
    Dependency(#[from] DependencyError),
}

#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Task {0} not found")]
    NotFound(String),
    #[error("Task {0} already exists")]
    AlreadyExists(String),
    #[error("Task creation failed. Cause: {0}")]
    CreationFailed(String),
    #[error("Task update failed. Cause: {0}")]
    UpdateFailed(String),
    #[error("Task deletion failed. Cause: {0}")]
    DeletionFailed(String),
    #[error("Task retrieval failed. Cause: {0}")]
    RetrievalFailed(String),
}

#[derive(Debug, thiserror::Error)]
pub enum DependencyError {
    #[error("Redis error: {0}")]
    RedisError(#[from] RedisError),
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Rabbit error: {0}")]
    RabbitError(#[from] lapin::Error),
    #[error("Actix error: {0}")]
    ActixError(#[from] actix_web::Error),
    #[error("Deadpool error: {0}")]
    DeadpoolError(#[from] deadpool_redis::PoolError),
}
