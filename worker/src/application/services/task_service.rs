use crate::application::dtos::task_dtos::NewTask;
use crate::domain::entities::task::Task;
use crate::domain::repositories::task_repository::TaskRepository;
use crate::shared::errors::Result;
use uuid::Uuid;

pub struct TaskService<T: TaskRepository> {
    task_repository: T,
}

impl<T: TaskRepository> TaskService<T> {
    pub fn new(task_repository: T) -> Self {
        Self { task_repository }
    }

    pub async fn save(&self, task: NewTask) -> Result<Uuid> {
        self.task_repository.save(task).await
    }

    pub async fn get(&self, id: Uuid) -> Option<Task> {
        self.task_repository.get(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::task_repository::MockTaskRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_save() {
        let mut mock = MockTaskRepository::new();
        mock.expect_save()
            .with(eq(NewTask {
                title: "test".to_string(),
            }))
            .times(1)
            .returning(|_| Ok(Uuid::new_v4()));

        let service = TaskService::new(mock);
        let result = service
            .save(NewTask {
                title: "test".to_string(),
            })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get() {
        let mut mock = MockTaskRepository::new();
        let id = Uuid::new_v4();
        mock.expect_get()
            .with(eq(id))
            .times(1)
            .returning(|_| Some(Task::new("test".to_string())));

        let service = TaskService::new(mock);
        let result = service.get(id).await;
        assert!(result.is_some());
    }
}
