use crate::domain::entities::task::TaskStatus;
use actix::Message;
use uuid::Uuid;
use crate::shared::errors::Result;

#[derive(Debug, Clone, PartialEq, Message)]
#[rtype(result = "Result<Uuid>")]
pub struct NewTask {
    pub title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateTask {
    pub id: Uuid,
    pub title: Option<String>,
    pub status: TaskStatus,
}
