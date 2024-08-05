use deadpool_redis::Pool;
use deadpool_redis::redis::AsyncCommands;
use redis::{RedisResult, transaction};
use uuid::Uuid;
use crate::application::dtos::task_dtos::{NewTask, UpdateTask};
use crate::domain::entities::task::{Task, TaskStatus};
use crate::domain::repositories::task_repository::TaskRepository;
use crate::infrastructure::db::connection::establish_connection;
use crate::shared::errors::{DependencyError, Result, TaskError};

#[derive(Clone)]
pub struct RedisTaskRepository {
    pool: Pool,
    hashset: String,
}

impl RedisTaskRepository {
    pub fn new(connection_string: &str, pool_size: usize, table: &str) -> Self {
        Self { pool: establish_connection(connection_string, pool_size), hashset: table.to_string() }
    }
}

impl TaskRepository for RedisTaskRepository {
    async fn save(&self, task: NewTask) -> Result<Uuid> {
        let mut conn = self.pool.get().await?;
        let task = Task::new(task.title);
        let task_string = serde_json::to_string(&task)?;
        let _ = conn.set(task.id.to_string(), task_string)
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