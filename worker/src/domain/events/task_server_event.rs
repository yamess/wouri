use std::sync::Arc;
use std::task::Poll;
use actix::Context;
use actix::dev::Stream;
use crate::domain::repositories::task_repository::TaskRepository;
use crate::domain::services::task_service::TaskService;

pub struct TaskServerEvent<T: TaskRepository> {
    pub service: TaskService<T>
}

impl <T: TaskRepository> TaskServerEvent<T> {
    pub fn new(task_repo: Arc<T>) -> Self {
        let service = TaskService::new(task_repo);
        Self { service }
    }
}

impl<T: TaskRepository> Stream for TaskServerEvent<T> {
    type Item = ();

    fn poll_next(&mut self, _: &mut Context<>) -> Poll<Option<Self::Item>, Self::Error> {
        Poll::Ready(Some(()))
    }
}