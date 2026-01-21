use crate::models::StatusEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RouterNic {
    pub id: Uuid,
    pub name: String,
    pub ip_address: String,
    pub network_name: Option<String>,
    pub subnet_name: Option<String>,
}

impl Default for RouterNic {
    fn default() -> Self {
        RouterNic {
            id: Uuid::nil(),
            name: String::new(),
            ip_address: String::new(),
            network_name: None,
            subnet_name: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum RouterStatus {
    #[serde(rename = "on")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for RouterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterStatus::Active => write!(f, "Active"),
            RouterStatus::Deleted => write!(f, "Deleted"),
            RouterStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

impl StatusEnum for RouterStatus {
    fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "on" => RouterStatus::Active,
            "deleted" => RouterStatus::Deleted,
            _ => RouterStatus::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Routers {
    pub id: Uuid,
    pub name: String,
    pub status: RouterStatus,
    pub bandwidth: u64,
    pub ip_address: String,
    pub availability_zone: Option<String>,
    pub created_at: String,
    pub attached_nics: Vec<RouterNic>,
}
