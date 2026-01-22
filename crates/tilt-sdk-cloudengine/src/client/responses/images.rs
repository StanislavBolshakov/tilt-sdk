use crate::models::common::extensible::LogSchemaWarnings;
use crate::models::{parse_datetime, ImageStatus, Images, NestedEntity, StatusEnum};
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

#[derive(Debug, Deserialize, Default)]
pub struct ImageResponse {
    #[serde(default)]
    pub list: Vec<ImageWrapper>,
    #[serde(default)]
    pub meta: ImagesMeta,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ImageWrapper {
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
    pub data: ImageData,
    #[serde(default)]
    pub created_row_dt: String,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ImageData {
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub config: ImageConfig,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ImageConfig {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub disk_format: String,
    #[serde(default)]
    pub os_distro: Option<String>,
    #[serde(default)]
    pub os_version: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub availability_zone: Option<NestedEntity<String>>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ImagesMeta {
    #[serde(default)]
    pub total_count: u32,
}

impl From<ImageWrapper> for Images {
    fn from(wrapper: ImageWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/compute/api/v1/projects/{project}/images");

        let data = wrapper.data;
        let config = data.config;
        let created_at = parse_datetime(&config.created_at).unwrap_or_else(chrono::Utc::now);
        let updated_at = config.updated_at.as_ref().and_then(|s| parse_datetime(s));

        Images {
            id: parse_uuid_or_warn(&config.id, "image.id"),
            name: config.name,
            size_bytes: config.size,
            disk_format: config.disk_format,
            os_distro: config.os_distro,
            os_version: config.os_version,
            status: ImageStatus::from_string(&config.status),
            visibility: config.visibility,
            availability_zone: config.availability_zone.map(|az| az.name),
            created_at,
            updated_at,
        }
    }
}

pub type ProjectImagesResponse = ImageResponse;
pub type SharedImagesResponse = ImageResponse;

pub type ProjectImageWrapper = ImageWrapper;
pub type SharedImageWrapper = ImageWrapper;

pub type ProjectImageData = ImageData;
pub type SharedImageData = ImageData;

pub type ProjectImageConfig = ImageConfig;
pub type SharedImageConfig = ImageConfig;
