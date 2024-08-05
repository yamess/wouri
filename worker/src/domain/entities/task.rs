use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub status: TaskStatus,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

#[derive(Debug, PartialEq, Serialize, Clone, Deserialize)]
pub enum TaskStatus {
    #[serde(rename = "not_started")]
    NotStarted,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            status: TaskStatus::NotStarted,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: None,
        }
    }
}
