use crate::models::common::extensible::LogSchemaWarnings;
use crate::models::{NestedEntity, StatusEnum};
use serde::Deserialize;

fn parse_uuid_or_warn(s: &str, field: &str) -> uuid::Uuid {
    match s.parse() {
        Ok(uuid) => uuid,
        Err(e) => {
            tracing::warn!(target: "tilt-sdk-cloudengine", "Invalid UUID for {}: {} ({})", field, s, e);
            uuid::Uuid::nil()
        }
    }
}

fn parse_uuid_optional(s: &str) -> Option<uuid::Uuid> {
    match s.parse() {
        Ok(uuid) => Some(uuid),
        Err(e) => {
            tracing::warn!(target: "tilt-sdk-cloudengine", "Invalid UUID: {} ({})", s, e);
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VolumeTypesResponse(pub Vec<VolumeTypeWrapper>);

#[derive(Debug, Clone, Deserialize, Default)]
pub struct VolumeTypeWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub extra_specs: Option<ExtraSpec>,
    #[serde(default)]
    pub is_public: Option<bool>,
    #[serde(default)]
    pub capabilities: Option<Capabilities>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ExtraSpec {
    #[serde(default)]
    #[serde(rename = "family")]
    pub family: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Capabilities {
    #[serde(default)]
    pub max_volume_size: Option<u64>,
    #[serde(default)]
    pub min_volume_size: Option<u64>,
}

pub type VolumesResponse = crate::ListResponse<VolumeWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct VolumeWrapper {
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub action_id: String,
    #[serde(default)]
    pub graph_id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub data: VolumeData,
    #[serde(default)]
    pub created_row_dt: String,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct VolumeData {
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub config: VolumeConfig,
    #[serde(default)]
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct VolumeConfig {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub bootable: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub attachments: Vec<VolumeAttachment>,
    #[serde(default)]
    pub volume_type: NestedEntity<uuid::Uuid>,
    #[serde(default)]
    pub availability_zone: Option<NestedEntity<String>>,
}

pub type NestedVolumeType = NestedEntity<uuid::Uuid>;
pub type NestedAZ = NestedEntity<String>;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct VolumeAttachment {
    #[serde(default)]
    pub server_id: String,
    #[serde(default)]
    pub device: String,
}

impl From<VolumeTypeWrapper> for crate::models::VolumeType {
    fn from(wrapper: VolumeTypeWrapper) -> Self {
        crate::models::VolumeType {
            id: wrapper.id,
            name: wrapper.name,
            description: wrapper.description,
        }
    }
}

impl From<VolumeWrapper> for crate::models::Volumes {
    fn from(wrapper: VolumeWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/order-service/api/v1/projects/{project}/volumes");

        let config = wrapper.data.config;

        let (attached_server_id, attached_server_name, device) =
            if let Some(attachment) = config.attachments.first() {
                let server_id = parse_uuid_optional(&attachment.server_id);
                (server_id, None, Some(attachment.device.clone()))
            } else {
                (None, None, None)
            };

        crate::models::Volumes {
            id: parse_uuid_or_warn(&config.id, "volume.id"),
            name: config.name,
            size: config.size,
            status: crate::models::VolumeStatus::from_string(&config.status),
            volume_type_id: config.volume_type.id,
            volume_type_name: Some(config.volume_type.name),
            availability_zone: config.availability_zone.map(|az| az.name),
            attached_server_id,
            attached_server_name,
            device,
            bootable: config.bootable == "true",
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }
}
