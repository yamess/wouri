use uuid::Uuid;
use crate::application::dtos::task_dtos::NewTask;
use crate::domain::entities::task::Task;
use crate::shared::errors::Result;

pub trait TaskRepository {
    async fn create_task(&self, task: Task) -> Result<Task>;
    async fn get_task(&self, task_id: Uuid) -> Option<Task>;
    async fn get_tasks(&self) -> Result<Vec<Task>>;
    async fn update_task(&self, id: Uuid, task: NewTask) -> Result<Task>;
    async fn delete_task(&self, id: Uuid) -> Result<()>;
}