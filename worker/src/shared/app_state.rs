use std::sync::Arc;
use crate::application::actors::task_actor::TaskActor;
use crate::domain::repositories::task_repository::TaskRepository;
use crate::infrastructure::actors::task_actor_pool::TaskActorPool;
use crate::infrastructure::configs::Config;

pub struct AppState {
    pub task_actor_pool: TaskActorPool<TaskActor<Box<dyn TaskRepository>>>,
    pub task_repository: Arc<Box<dyn TaskRepository>>,
    pub config: Config
}

impl AppState  {
    pub fn new() -> Self {
        let config = Config::new();
        let task_repository = Arc::new(Box::new(TaskRepository::new()));
        let task_actor_pool = TaskActorPool::new(config.task_actor_pool_size, task_repository.clone());
        Self {
    }
}