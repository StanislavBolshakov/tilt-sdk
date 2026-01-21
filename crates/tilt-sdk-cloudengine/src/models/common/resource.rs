use super::StatusEnum;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Regions {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityZone {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementPolicy {
    pub id: Uuid,
    pub name: String,
    pub policy_type: Option<String>,
    pub availability_zone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tasks {
    pub id: Uuid,
    pub status: TaskStatus,
    pub object_type: String,
    pub object_id: Uuid,
    pub initiator: String,
    pub created_at: DateTime<Utc>,
    pub duration_seconds: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unknown")]
    Unknown,
}

impl StatusEnum for TaskStatus {
    fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "new" => TaskStatus::New,
            "running" => TaskStatus::Running,
            "successful" | "success" => TaskStatus::Successful,
            "failed" | "error" => TaskStatus::Failed,
            _ => TaskStatus::Unknown,
        }
    }
}
