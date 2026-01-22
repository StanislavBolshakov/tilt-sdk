use crate::models::common::extensible::LogSchemaWarnings;
use crate::models::ListResponse;
use serde::Deserialize;

pub type FlavorsResponse = ListResponse<FlavorWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct FlavorWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub ram: u64,
    #[serde(default)]
    pub vcpus: u64,
    #[serde(default)]
    pub gpus: u64,
    #[serde(default)]
    pub extra_specs: Option<ExtraSpec>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<FlavorWrapper> for crate::models::Flavors {
    fn from(wrapper: FlavorWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/compute/api/v1/projects/{project}/flavors");

        crate::models::Flavors {
            id: wrapper.id,
            name: wrapper.name,
            description: wrapper.description,
            ram_mb: wrapper.ram,
            vcpus: wrapper.vcpus,
            gpus: wrapper.gpus,
            extra_specs: wrapper.extra_specs.map(|es| crate::models::ExtraSpec {
                family: es.family,
                series: es.series,
                hardware_group: es.hardware_group,
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ExtraSpec {
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub series: Option<String>,
    #[serde(default)]
    pub hardware_group: Option<String>,
}
