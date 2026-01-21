use crate::StatusEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volumes {
    pub id: Uuid,
    pub name: String,
    pub size: u64,
    pub status: VolumeStatus,
    pub volume_type_id: Uuid,
    pub volume_type_name: Option<String>,
    pub availability_zone: Option<String>,
    pub attached_server_id: Option<Uuid>,
    pub attached_server_name: Option<String>,
    pub device: Option<String>,
    pub bootable: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VolumeStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "in-use")]
    InUse,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "error_deleting")]
    ErrorDeleting,
    #[serde(rename = "attaching")]
    Attaching,
    #[serde(rename = "detaching")]
    Detaching,
    #[serde(rename = "unknown")]
    Unknown,
}

impl StatusEnum for VolumeStatus {
    fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "available" => VolumeStatus::Available,
            "in-use" => VolumeStatus::InUse,
            "creating" => VolumeStatus::Creating,
            "deleting" => VolumeStatus::Deleting,
            "error" => VolumeStatus::Error,
            "error_deleting" => VolumeStatus::ErrorDeleting,
            "attaching" => VolumeStatus::Attaching,
            "detaching" => VolumeStatus::Detaching,
            _ => VolumeStatus::Unknown,
        }
    }
}
