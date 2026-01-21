use serde::Deserialize;

pub type RegionsResponse = Vec<RegionWrapper>;

#[derive(Debug, Clone, Deserialize)]
pub struct RegionWrapper {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

impl From<RegionWrapper> for crate::models::Regions {
    fn from(wrapper: RegionWrapper) -> Self {
        crate::models::Regions {
            id: wrapper.id,
            name: wrapper.name,
            description: wrapper.description,
        }
    }
}
