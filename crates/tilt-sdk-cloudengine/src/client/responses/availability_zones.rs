use crate::log_schema_drift;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AvailabilityZonesResponse(pub Vec<AvailabilityZoneWrapper>);

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct AvailabilityZoneWrapper {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<AvailabilityZoneWrapper> for crate::models::AvailabilityZone {
    fn from(wrapper: AvailabilityZoneWrapper) -> Self {
        log_schema_drift!(
            wrapper,
            "/compute/api/v1/projects/{project}/availability-zones"
        );

        crate::models::AvailabilityZone {
            id: wrapper.id,
            name: wrapper.name,
            description: wrapper.description,
        }
    }
}
