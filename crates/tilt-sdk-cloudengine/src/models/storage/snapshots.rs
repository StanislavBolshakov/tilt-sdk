use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Snapshots {
    pub id: Uuid,
    pub name: String,
    pub size: i32,
    pub status: String,
    pub volume_id: Uuid,
    pub volume_name: Option<String>,
    pub volume_type_id: Option<Uuid>,
    pub volume_type_name: Option<String>,
    pub availability_zone: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub description: Option<String>,
}

pub type Snapshot = Snapshots;
