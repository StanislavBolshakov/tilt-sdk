use crate::impl_table_row;

#[derive(Debug)]
pub struct VolumeRow {
    pub id: String,
    pub name: String,
    pub size: String,
    pub status: String,
    pub az: String,
}

impl_table_row!(VolumeRow, id, name, size, status, az);

#[derive(Debug)]
pub struct VolumeRowLong {
    pub id: String,
    pub name: String,
    pub size: String,
    pub status: String,
    pub az: String,
    pub attached_to: String,
    pub volume_type: String,
    pub bootable: String,
    pub device: String,
    pub created_at: String,
}

impl_table_row!(
    VolumeRowLong,
    id,
    name,
    size,
    status,
    az,
    attached_to,
    volume_type,
    bootable,
    device,
    created_at
);

#[derive(Debug)]
pub struct VolumeShowRow {
    pub id: String,
    pub name: String,
    pub size: String,
    pub status: String,
    pub volume_type: String,
    pub az: String,
    pub bootable: String,
    pub attached_server_id: String,
    pub device: String,
    pub created_at: String,
    pub updated_at: String,
}

impl_table_row!(
    VolumeShowRow,
    id,
    name,
    size,
    status,
    volume_type,
    az,
    bootable,
    attached_server_id,
    device,
    created_at,
    updated_at
);

#[derive(Debug)]
pub struct SnapshotRow {
    pub id: String,
    pub name: String,
    pub size: String,
    pub status: String,
    pub volume: String,
    pub created: String,
}

impl_table_row!(SnapshotRow, id, name, size, status, volume, created);

#[derive(Debug)]
pub struct BackupRow {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub state: String,
    pub last_backup: String,
    pub size_gb: String,
    pub availability_zone: String,
}

impl_table_row!(
    BackupRow,
    id,
    name,
    type_,
    state,
    last_backup,
    size_gb,
    availability_zone
);

#[derive(Debug)]
pub struct BackupRowLong {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub state: String,
    pub order_id: String,
    pub last_backup: String,
    pub size_gb: String,
    pub availability_zone: String,
    pub provider: String,
    pub created: String,
}

impl_table_row!(
    BackupRowLong,
    id,
    name,
    type_,
    state,
    order_id,
    last_backup,
    size_gb,
    availability_zone,
    provider,
    created
);
