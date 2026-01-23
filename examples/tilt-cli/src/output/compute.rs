use crate::impl_table_row;

#[derive(Debug)]
pub struct ServersRow {
    pub id: String,
    pub name: String,
    pub status: String,
    pub availability_zone: String,
}

impl_table_row!(ServersRow, id, name, status, availability_zone);

#[derive(Debug)]
pub struct ServersRowLong {
    pub id: String,
    pub name: String,
    pub status: String,
    pub flavor: String,
    pub image: String,
    pub ip: String,
    pub availability_zone: String,
    pub created_at: String,
}

impl_table_row!(
    ServersRowLong,
    id,
    name,
    status,
    flavor,
    image,
    ip,
    availability_zone,
    created_at
);

#[derive(Debug)]
pub struct FlavorsRow {
    pub id: String,
    pub name: String,
    pub ram_mb: String,
    pub vcpus: String,
    pub gpus: String,
}

impl_table_row!(FlavorsRow, id, name, ram_mb, vcpus, gpus);

#[derive(Debug)]
pub struct ImageRow {
    pub id: String,
    pub name: String,
    pub size_gb: String,
    pub status: String,
    pub created: String,
}

impl_table_row!(ImageRow, id, name, size_gb, status, created);

#[derive(Debug)]
pub struct ImageRowLong {
    pub id: String,
    pub name: String,
    pub size_gb: String,
    pub status: String,
    pub disk_format: String,
    pub os_distro: String,
    pub os_version: String,
    pub visibility: String,
    pub created_at: String,
}

impl_table_row!(
    ImageRowLong,
    id,
    name,
    size_gb,
    status,
    disk_format,
    os_distro,
    os_version,
    visibility,
    created_at
);

#[derive(Debug)]
pub struct ImageShowRow {
    pub id: String,
    pub name: String,
    pub size_gb: String,
    pub status: String,
    pub disk_format: String,
    pub os_distro: String,
    pub os_version: String,
    pub visibility: String,
    pub availability_zone: String,
    pub created_at: String,
    pub updated_at: String,
}

impl_table_row!(
    ImageShowRow,
    id,
    name,
    size_gb,
    status,
    disk_format,
    os_distro,
    os_version,
    visibility,
    availability_zone,
    created_at,
    updated_at
);

#[derive(Debug)]
pub struct TaskRow {
    pub id: String,
    pub object_type: String,
    pub object_id: String,
    pub status: String,
    pub initiator: String,
    pub created_at: String,
    pub duration_s: String,
}

impl_table_row!(
    TaskRow,
    id,
    object_type,
    object_id,
    status,
    initiator,
    created_at,
    duration_s
);

#[derive(Debug)]
pub struct TaskRowDetail {
    pub id: String,
    pub object_type: String,
    pub object_id: String,
    pub status: String,
    pub initiator: String,
    pub created_at: String,
    pub duration_s: String,
    pub error: String,
}

impl_table_row!(
    TaskRowDetail,
    id,
    object_type,
    object_id,
    status,
    initiator,
    created_at,
    duration_s,
    error
);

#[derive(Debug)]
pub struct RegionRow {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl_table_row!(RegionRow, id, name, description);

#[derive(Debug)]
pub struct PlacementRow {
    pub id: String,
    pub name: String,
    pub policy_type: String,
    pub availability_zone: String,
}

impl_table_row!(PlacementRow, id, name, policy_type, availability_zone);
