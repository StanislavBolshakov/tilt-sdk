use crate::models::{ImageStatus, Images, NestedEntity, StatusEnum};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ImageWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub disk_format: Option<String>,
    #[serde(default)]
    pub os_distro: Option<String>,
    #[serde(default)]
    pub os_version: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub availability_zone: Option<NestedEntity<String>>,
}

impl From<ImageWrapper> for Images {
    fn from(wrapper: ImageWrapper) -> Self {
        Images {
            id: wrapper.id,
            name: wrapper.name,
            size_bytes: wrapper.size.unwrap_or(0),
            disk_format: wrapper.disk_format.unwrap_or_default(),
            os_distro: wrapper.os_distro,
            os_version: wrapper.os_version,
            status: ImageStatus::from_string(wrapper.status.as_deref().unwrap_or("unknown")),
            visibility: wrapper.visibility,
            availability_zone: wrapper.availability_zone.map(|az| az.name),
            created_at: chrono::DateTime::UNIX_EPOCH,
            updated_at: None,
        }
    }
}
