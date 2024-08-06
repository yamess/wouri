use crate::application::dtos::task_dtos::NewTask;
use crate::domain::entities::task::Task;
use crate::domain::repositories::task_repository::TaskRepository;
use crate::infrastructure::db::connection::establish_connection;
use crate::shared::errors::{DependencyError, Result, TaskError};
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Pool;

use uuid::Uuid;

#[derive(Clone)]
pub struct RedisTaskRepository {
    pool: Pool,
}

impl RedisTaskRepository {
    pub fn new(connection_string: &str) -> Self {
        Self {
            pool: establish_connection(connection_string),
        }
    }
}

impl TaskRepository for RedisTaskRepository {
    async fn save(&self, task: NewTask) -> Result<Uuid> {
        let mut conn = self.pool.get().await.map_err(|e| {
            log::error!("Failed to get connection from redis pool: {}", e);
            return DependencyError::DeadpoolError(e);
        })?;
        let task = Task::new(task.title);
        let task_string = serde_json::to_string(&task).map_err(|e| {
            log::error!("Failed to serialize task: {}", e);
            return DependencyError::SerdeJsonError(e);
        })?;
        let _ = conn
            .set(task.id.to_string(), task_string)
            .await
            .map_err(|e| {
                log::error!("Failed to set task in redis: {}", e);
                return TaskError::CreationFailed(e.to_string());
            })?;
        Ok(task.id)
    }

    async fn get(&self, id: Uuid) -> Option<Task> {
        let mut conn = self.pool.get().await.ok()?;
        let task_string: String = conn
            .get(id.to_string())
            .await
            .map_err(|e| {
                log::error!("Failed to get task from redis: {}", e);
                return TaskError::RetrievalFailed(e.to_string());
            })
            .ok()?;

        let task: Task = serde_json::from_str(task_string.as_str()).ok()?;
        Some(task)
    }
}
