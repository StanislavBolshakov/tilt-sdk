use crate::log_schema_drift;
use crate::models::ListResponse;
use serde::Deserialize;

pub type PlacementPoliciesResponse = ListResponse<PlacementPolicyWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct PlacementPolicyWrapper {
    #[serde(default)]
    #[serde(rename = "item_id")]
    pub id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "created_row_dt")]
    pub created_time: String,
    #[serde(default)]
    pub data: PlacementPolicyDataWrapper,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<PlacementPolicyWrapper> for crate::models::PlacementPolicy {
    fn from(wrapper: PlacementPolicyWrapper) -> Self {
        log_schema_drift!(
            wrapper,
            "/compute/api/v1/projects/{project}/placement-policies"
        );

        crate::models::PlacementPolicy {
            id: wrapper.id,
            name: wrapper.data.config.name,
            policy_type: wrapper.data.config.policy_type,
            availability_zone: wrapper.data.config.availability_zone.map(|az| az.name),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlacementPolicyDataWrapper {
    pub state: String,
    pub config: PlacementPolicyConfigWrapper,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlacementPolicyConfigWrapper {
    pub name: String,
    pub policy_type: Option<String>,
    #[serde(rename = "availability_zone")]
    pub availability_zone: Option<PlacementAzWrapper>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlacementAzWrapper {
    pub id: String,
    pub name: String,
    pub description: String,
}
