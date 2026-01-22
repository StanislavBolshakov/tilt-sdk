use crate::models::common::extensible::LogSchemaWarnings;
use crate::models::StatusEnum;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ImageUploadsResponse {
    #[serde(default)]
    pub list: Vec<ImageUploadWrapper>,
    #[serde(default)]
    pub meta: ImageUploadsMeta,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct ImageUploadWrapper {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub os_distro: Option<String>,
    #[serde(default)]
    pub os_version: Option<String>,
    #[serde(default)]
    pub create_dt: String,
    #[serde(default)]
    pub update_dt: String,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<ImageUploadWrapper> for crate::models::ImageUpload {
    fn from(wrapper: ImageUploadWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/compute/api/v1/projects/{project}/image-uploads");

        crate::models::ImageUpload {
            id: wrapper.id.parse().unwrap_or_default(),
            name: wrapper.name,
            status: crate::models::ImageUploadStatus::from_string(&wrapper.status),
            os_distro: wrapper.os_distro,
            os_version: wrapper.os_version,
            create_dt: wrapper.create_dt,
            update_dt: wrapper.update_dt,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct ImageUploadsMeta {
    #[serde(default)]
    pub total_count: u32,
    #[serde(default)]
    pub previous: Option<String>,
    #[serde(default)]
    pub next: Option<String>,
}
