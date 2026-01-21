use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flavors {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ram_mb: u64,
    pub vcpus: u64,
    pub gpus: u64,
    pub extra_specs: Option<ExtraSpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtraSpec {
    pub family: Option<String>,
    pub series: Option<String>,
    pub hardware_group: Option<String>,
}
