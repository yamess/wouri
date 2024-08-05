use crate::domain::entities::task::TaskStatus;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct NewTask {
    pub title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateTask {
    pub id: Uuid,
    pub title: Option<String>,
    pub status: TaskStatus,
}
