use crate::application::dtos::task_dtos::NewTask;
use crate::application::services::task_service::TaskService;
use crate::domain::repositories::task_repository::TaskRepository;
use crate::shared::errors::Result;
use actix::ActorFutureExt;
use actix::{
    Actor, Context, ContextFutureSpawner, Handler, ResponseActFuture, Running, WrapFuture,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskActor<T: TaskRepository> {
    pub task_repo: T,
}

impl<T: TaskRepository> TaskActor<T> {
    pub fn new(task_repo: T) -> Self {
        Self { task_repo }
    }
}

impl<T: TaskRepository + Unpin + Clone + 'static> Actor for TaskActor<T> {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("TaskActor started");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        log::info!("TaskActor stopping");
        Running::Stop
    }
}

impl<T: TaskRepository + Unpin + Clone + 'static> Handler<NewTask> for TaskActor<T> {
    type Result = ResponseActFuture<Self, Result<Uuid>>;

    fn handle(&mut self, msg: NewTask, _ctx: &mut Self::Context) -> Self::Result {
        let service = TaskService::<T>::new(self.task_repo.clone());
        Box::pin(
            async move { service.save(msg).await }
                .into_actor(self)
                .map(|res, _, _| res),
        )
    }
}
