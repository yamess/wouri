use crate::domain::entities::task::Task;
use uuid::Uuid;
use crate::shared::errors::Result;

pub trait TaskRepository {
    pub async fn add(task: Task) -> Result<()>;
    
    pub async fn get(task_id: Uuid) -> Option<Task>;
}

