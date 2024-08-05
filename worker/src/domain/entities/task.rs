use uuid::Uuid
use serde::Serialize


#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub startime: i64,
    pub status: TaskStatus
}
