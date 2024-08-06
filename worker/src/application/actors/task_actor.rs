use std::sync::Arc;
use crate::application::dtos::task_dtos::{GetTask, NewTask};
use crate::domain::services::task_service::TaskService;
use crate::domain::repositories::task_repository::TaskRepository;
use crate::shared::errors::Result;
use actix::ActorFutureExt;
use actix::{
    Actor, Context, Handler, ResponseActFuture, Running, WrapFuture,
};
use uuid::Uuid;
use crate::domain::entities::task::Task;

#[derive(Debug, Clone)]
pub struct TaskActor<T: TaskRepository> {
    pub task_repo: Arc<T>,
    pub name: String
}

impl<T: TaskRepository> TaskActor<T> {
    pub fn new(task_repo: Arc<T>, name: String) -> Self {
        Self { task_repo, name }
    }
}

impl<T: TaskRepository + Unpin + 'static> Actor for TaskActor<T> {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Actor {} started", self.name);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        log::info!("Actor {} stopping", self.name);
        Running::Stop
    }
}

impl<T: TaskRepository + Unpin + 'static> Handler<NewTask> for TaskActor<T> {
    type Result = ResponseActFuture<Self, Result<Uuid>>;

    fn handle(&mut self, msg: NewTask, _ctx: &mut Self::Context) -> Self::Result {
        let service = TaskService::<T>::new(self.task_repo.clone());
        log::info!("Actor {} handling NewTask", self.name);
        Box::pin(
            async move { service.save(msg).await }
                .into_actor(self)
                .map(|res, _, _| res),
        )
    }
}

impl<T: TaskRepository + Unpin + Clone + 'static> Handler<GetTask> for TaskActor<T> {
    type Result = ResponseActFuture<Self, Option<Task>>;

    fn handle(&mut self, msg: GetTask, _ctx: &mut Self::Context) -> Self::Result {
        let service = TaskService::<T>::new(self.task_repo.clone());
        Box::pin(
            async move { service.get(msg.id).await }
                .into_actor(self)
                .map(|res, _, _| res),
        )
    }
}