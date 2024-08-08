use crate::application::actors::task_actor::TaskActor;
use crate::domain::repositories::task_repository::TaskRepository;
use actix::{Actor, Addr};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct TaskActorPool<T: TaskRepository + Unpin + 'static> {
    actors: Vec<Addr<TaskActor<T>>>,
    index: Arc<Mutex<usize>>,
}

impl<T: TaskRepository + Unpin + 'static> TaskActorPool<T> {
    pub fn new(size: usize, task_repo: Arc<T>) -> Self {
        let mut actors = Vec::with_capacity(size);

        // Create actors in new arbiter and store their addresses
        for _ in 0..size {
            let name = format!("task-actor-{}", actors.len());
            let arbiter = actix::Arbiter::new().handle();
            let actor = TaskActor::new(task_repo.clone(), name);
            let addr = TaskActor::start_in_arbiter(&arbiter, |_| actor);
            actors.push(addr);
        }
        Self {
            actors,
            index: Arc::new(Mutex::new(0)),
        }
    }

    pub fn get_actor(&self) -> Addr<TaskActor<T>> {
        let mut index = self.index.lock().unwrap();
        let actor = self.actors[*index].clone();
        *index = (*index + 1) % self.actors.len();
        actor
    }
}
