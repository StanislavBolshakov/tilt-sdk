use crate::models::common::extensible::LogSchemaWarnings;
use serde::Deserialize;

pub type RegionsResponse = Vec<RegionWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct RegionWrapper {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<RegionWrapper> for crate::models::Regions {
    fn from(wrapper: RegionWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/compute/api/v1/projects/{project}/regions");

        crate::models::Regions {
            id: wrapper.id,
            name: wrapper.name,
            description: wrapper.description,
        }
    }
}
