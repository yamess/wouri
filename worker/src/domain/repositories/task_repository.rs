use crate::application::dtos::task_dtos::NewTask;
use crate::domain::entities::task::Task;
use crate::shared::errors::Result;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
pub trait TaskRepository: Send + Sync {
    async fn save(&self, task: NewTask) -> Result<Uuid>;
    async fn get(&self, id: Uuid) -> Option<Task>;
    // async fn get_tasks(&self) -> Result<Vec<Task>>;
    // async fn update_task(&self, id: Uuid, task: UpdateTask) -> Result<Task>;
    // async fn delete_task(&self, id: Uuid) -> Result<()>;
}
