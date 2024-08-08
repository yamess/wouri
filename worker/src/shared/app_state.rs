use crate::infrastructure::actors::task_actor_pool::TaskActorPool;
use crate::infrastructure::configs::Config;
use crate::infrastructure::repositories::redis_task_repository::RedisTaskRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub task_actor_pool: TaskActorPool<RedisTaskRepository>,
    pub task_repository: Arc<RedisTaskRepository>,
    pub config: Config,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::new();
        let redis_task_repository = Arc::new(RedisTaskRepository::new(
            &config.redis.redis_connection_string,
        ));
        let task_actor_pool =
            TaskActorPool::new(config.app.task_pool_size, redis_task_repository.clone());
        Self {
            task_actor_pool,
            task_repository: redis_task_repository,
            config,
        }
    }
}
