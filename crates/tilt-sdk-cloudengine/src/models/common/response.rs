use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ListMeta {
    #[serde(default)]
    pub total_count: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListResponse<T> {
    #[serde(default)]
    pub list: Vec<T>,
    #[serde(default)]
    pub meta: ListMeta,
}

impl<T> ListResponse<T> {
    pub fn into_parts(self) -> (Vec<T>, u32) {
        (self.list, self.meta.total_count)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NestedEntity<T = String> {
    #[serde(default)]
    pub id: T,
    #[serde(default)]
    pub name: String,
}

impl<T> NestedEntity<T> {
    pub fn new(id: T, name: String) -> Self {
        Self { id, name }
    }
}
