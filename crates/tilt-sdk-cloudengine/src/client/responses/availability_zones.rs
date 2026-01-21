use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AvailabilityZonesResponse(pub Vec<AvailabilityZoneWrapper>);

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AvailabilityZoneWrapper {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl From<AvailabilityZoneWrapper> for crate::models::AvailabilityZone {
    fn from(wrapper: AvailabilityZoneWrapper) -> Self {
        crate::models::AvailabilityZone {
            id: wrapper.id,
            name: wrapper.name,
            description: wrapper.description,
        }
    }
}
