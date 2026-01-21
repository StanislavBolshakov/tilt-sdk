use crate::StatusEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instances {
    pub id: Uuid,
    pub name: String,
    pub status: InstanceStatus,
    pub flavor_id: Uuid,
    pub flavor_name: Option<String>,
    pub flavor_vcpus: Option<u64>,
    pub flavor_ram_mb: Option<u64>,
    pub flavor_gpus: Option<u64>,
    pub image_id: Option<Uuid>,
    pub image_name: Option<String>,
    pub availability_zone: Option<String>,
    pub all_addresses: Vec<String>,
    pub created_row_dt: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InstanceStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "building")]
    Building,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "poweroff")]
    PowerOff,
    #[serde(rename = "rescue")]
    Rescue,
    #[serde(rename = "resize")]
    Resize,
    #[serde(rename = "reverted_resize")]
    RevertedResize,
    #[serde(rename = "shelved")]
    Shelved,
    #[serde(rename = "shelved_offloaded")]
    ShelvedOffloaded,
    #[serde(rename = "soft_deleted")]
    SoftDeleted,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "verify_resize")]
    VerifyResize,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "unknown")]
    Unknown,
}

impl StatusEnum for InstanceStatus {
    fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" | "on" => InstanceStatus::Active,
            "building" => InstanceStatus::Building,
            "paused" => InstanceStatus::Paused,
            "poweroff" | "power_off" | "off" => InstanceStatus::PowerOff,
            "rescue" => InstanceStatus::Rescue,
            "resize" => InstanceStatus::Resize,
            "reverted_resize" => InstanceStatus::RevertedResize,
            "shelved" => InstanceStatus::Shelved,
            "shelved_offloaded" => InstanceStatus::ShelvedOffloaded,
            "soft_deleted" | "soft-delete" => InstanceStatus::SoftDeleted,
            "suspended" => InstanceStatus::Suspended,
            "verify_resize" => InstanceStatus::VerifyResize,
            "error" => InstanceStatus::Error,
            _ => InstanceStatus::Unknown,
        }
    }
}
