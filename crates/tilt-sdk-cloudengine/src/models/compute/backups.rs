use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Backup {
    pub id: Uuid,
    pub order_id: Uuid,
    pub action_id: Uuid,
    pub graph_id: Uuid,
    pub backup_type: String,
    pub state: String,
    pub source_object_id: Uuid,
    pub source_object_name: String,
    pub source_object_type: String,
    pub last_backup_time: Option<String>,
    pub availability_zone_id: String,
    pub availability_zone_name: String,
    pub size_gb: f64,
    pub provider: String,
    pub created_time: String,
}

pub type Backups = Backup;
