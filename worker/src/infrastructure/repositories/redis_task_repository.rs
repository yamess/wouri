use deadpool_redis::Pool;
use deadpool_redis::redis::AsyncCommands;
use uuid::Uuid;
use crate::application::dtos::task_dtos::NewTask;
use crate::domain::entities::task::{Task, TaskStatus};
use crate::domain::repositories::task_repository::TaskRepository;
use crate::infrastructure::db::connection::establish_connection;
use crate::shared::errors::Result;

#[derive(Clone)]
pub struct RedisTaskRepository {
    pool: Pool,
}

impl RedisTaskRepository {
    pub fn new(connection_string: &str, pool_size: usize) -> Self {
        Self { pool: establish_connection(connection_string, pool_size) }
    }
}

impl TaskRepository for RedisTaskRepository {
    async fn create_task(&self, task: NewTask) -> Result<Task> {
        let task = Task {
            id: Uuid::new_v4(),
            title: task.title,
            description: task.description,
            status: TaskStatus::NotStarted,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: None,
        };
        let mut conn = self.pool.get().await?;
        let result = conn.sadd("tasks", task.id.to_string()).await;
        if result == 0 {
            return Err("Task already exists".into());
        }
    }

    async fn get_task(&self, task_id: Uuid) -> Option<Task> {
        unimplemented!()
    }

    async fn get_tasks(&self) -> Result<Vec<Task>> {
        unimplemented!()
    }

    async fn update_task(&self, id: Uuid, task: NewTask) -> Result<Task> {
        unimplemented!()
    }

    async fn delete_task(&self, id: Uuid) -> Result<()> {
        todo!()
    }
}