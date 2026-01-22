use crate::models::common::extensible::LogSchemaWarnings;
use crate::models::compute::Backup;
use crate::models::ListResponse;
use serde::Deserialize;

pub type BackupsResponse = ListResponse<BackupWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct BackupWrapper {
    #[serde(default)]
    #[serde(rename = "item_id")]
    pub id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "order_id")]
    pub order_id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "action_id")]
    pub action_id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "graph_id")]
    pub graph_id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "type")]
    pub backup_type: String,
    #[serde(default)]
    pub data: BackupDataWrapper,
    #[serde(default)]
    #[serde(rename = "created_row_dt")]
    pub created_time: String,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<BackupWrapper> for Backup {
    fn from(wrapper: BackupWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/compute/api/v1/projects/{project}/backups");

        Backup {
            id: wrapper.id,
            order_id: wrapper.order_id,
            action_id: wrapper.action_id,
            graph_id: wrapper.graph_id,
            backup_type: wrapper.backup_type,
            state: wrapper.data.state,
            source_object_id: wrapper.data.config.source_object.id,
            source_object_name: wrapper.data.config.source_object.name,
            source_object_type: wrapper.data.config.source_object.object_type,
            last_backup_time: wrapper.data.config.last_backup_time,
            availability_zone_id: wrapper.data.config.availability_zone.id,
            availability_zone_name: wrapper.data.config.availability_zone.name,
            size_gb: wrapper.data.config.size_gb,
            provider: wrapper.data.provider,
            created_time: wrapper.created_time,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BackupDataWrapper {
    pub state: String,
    pub config: BackupConfigWrapper,
    pub provider: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BackupConfigWrapper {
    #[serde(rename = "source_object")]
    pub source_object: BackupSourceObject,
    #[serde(rename = "last_backup_dt")]
    pub last_backup_time: Option<String>,
    #[serde(rename = "availability_zone")]
    pub availability_zone: BackupAzWrapper,
    #[serde(rename = "sum_backup_sizes_gb")]
    pub size_gb: f64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BackupSourceObject {
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BackupAzWrapper {
    pub id: String,
    pub name: String,
    pub description: String,
}
