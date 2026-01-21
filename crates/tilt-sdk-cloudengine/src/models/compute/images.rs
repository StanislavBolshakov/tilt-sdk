use crate::StatusEnum;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Images {
    pub id: Uuid,
    pub name: String,
    pub size_bytes: u64,
    pub disk_format: String,
    pub os_distro: Option<String>,
    pub os_version: Option<String>,
    pub status: ImageStatus,
    pub visibility: Option<String>,
    pub availability_zone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "saving")]
    Saving,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "killed")]
    Killed,
    #[serde(rename = "pending_delete")]
    PendingDelete,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "unknown")]
    Unknown,
}

impl StatusEnum for ImageStatus {
    fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" => ImageStatus::Active,
            "saving" => ImageStatus::Saving,
            "queued" => ImageStatus::Queued,
            "killed" => ImageStatus::Killed,
            "pending_delete" => ImageStatus::PendingDelete,
            "error" => ImageStatus::Error,
            _ => ImageStatus::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageUpload {
    pub id: Uuid,
    pub name: String,
    pub status: ImageUploadStatus,
    pub os_distro: Option<String>,
    pub os_version: Option<String>,
    pub create_dt: String,
    pub update_dt: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageUploadStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unknown")]
    Unknown,
}

impl StatusEnum for ImageUploadStatus {
    fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" => ImageUploadStatus::Active,
            "deleted" => ImageUploadStatus::Deleted,
            "pending" | "uploading" => ImageUploadStatus::Pending,
            "failed" | "error" => ImageUploadStatus::Failed,
            _ => ImageUploadStatus::Unknown,
        }
    }
}
