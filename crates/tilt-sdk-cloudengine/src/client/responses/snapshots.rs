use crate::models::{ListResponse, NestedEntity};
use serde::Deserialize;

pub type SnapshotsResponse = ListResponse<SnapshotWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SnapshotWrapper {
    #[serde(default)]
    pub item_id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(default)]
    pub data: SnapshotDataWrapper,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SnapshotDataWrapper {
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub config: SnapshotConfigWrapper,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SnapshotConfigWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size: i32,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub volume_id: uuid::Uuid,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub volume_type: Option<NestedEntity<uuid::Uuid>>,
    #[serde(default)]
    pub availability_zone: Option<NestedEntity<String>>,
}

pub type VolumeTypeWrapper = NestedEntity<uuid::Uuid>;

pub type AvailabilityZoneWrapper = NestedEntity<String>;

impl From<SnapshotWrapper> for crate::models::Snapshots {
    fn from(wrapper: SnapshotWrapper) -> Self {
        let config = &wrapper.data.config;
        crate::models::Snapshots {
            id: wrapper.item_id,
            name: config.name.clone(),
            size: config.size,
            status: config.status.clone(),
            volume_id: config.volume_id,
            volume_name: None,
            volume_type_id: config.volume_type.as_ref().map(|vt| vt.id),
            volume_type_name: config.volume_type.as_ref().map(|vt| vt.name.clone()),
            availability_zone: config.availability_zone.as_ref().map(|az| az.name.clone()),
            created_at: config.created_at.clone(),
            updated_at: config.updated_at.clone(),
            description: config.description.clone(),
        }
    }
}
