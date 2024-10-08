use crate::domain::entities::task::Task;
use crate::domain::entities::task::TaskStatus;
use crate::shared::errors::Result;
use actix::Message;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Message, Deserialize)]
#[rtype(result = "Result<Uuid>")]
pub struct NewTask {
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Message)]
#[rtype(result = "Result<Task>")]
pub struct UpdateTask {
    pub id: Uuid,
    pub title: Option<String>,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, PartialEq, Message)]
#[rtype(result = "Result<()>")]
pub struct DeleteTask {
    pub id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Message)]
#[rtype(result = "Option<Task>")]
pub struct GetTask {
    pub id: Uuid,
}
