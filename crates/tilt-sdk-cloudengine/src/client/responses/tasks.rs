use crate::models::StatusEnum;
use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct TasksResponse {
    #[serde(default)]
    pub list: Vec<TaskWrapper>,
}

#[derive(Debug, Deserialize, Default)]
pub struct TaskResponse {
    #[serde(default)]
    pub data: TaskWrapper,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TaskWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub object_type: String,
    #[serde(default)]
    pub object_id: uuid::Uuid,
    #[serde(default)]
    pub action_id: uuid::Uuid,
    #[serde(default)]
    pub initiator: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: Option<String>,
    #[serde(default)]
    pub expires: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub error: Option<serde_json::Value>,
}

impl From<TaskWrapper> for crate::models::Tasks {
    fn from(wrapper: TaskWrapper) -> Self {
        crate::models::Tasks {
            id: wrapper.id,
            status: crate::models::TaskStatus::from_string(&wrapper.status),
            object_type: wrapper.object_type,
            object_id: wrapper.object_id,
            initiator: wrapper.initiator,
            created_at: wrapper.created.parse().unwrap_or_else(|_| Utc::now()),
            duration_seconds: None,
            error: wrapper.error.and_then(|e| serde_json::from_value(e).ok()),
        }
    }
}
