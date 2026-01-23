use cloudengine::{ComputeError, Service, Snapshots, Volumes};
use tilt_sdk_cloudengine as cloudengine;

use crate::output::{
    BackupRow, BackupRowLong, SnapshotRow, VolumeRow, VolumeRowLong, VolumeShowRow, format_date, format_opt, format_opt_ref, format_table,
};
use cloudengine::models::Backups;

pub async fn list_volumes(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<Volumes>, ComputeError> {
    client.list_volumes(limit, page).await
}

pub async fn list_snapshots(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<Snapshots>, ComputeError> {
    client.list_snapshots(limit, page).await
}

pub async fn show_volume(
    client: &cloudengine::ComputeClient<'_>,
    volume_id: &str,
) -> Result<Volumes, ComputeError> {
    let id = volume_id.parse().map_err(|e| {
        ComputeError::new(
            Service::OrderService,
            None,
            format!("Invalid volume ID: {}", e),
        )
    })?;
    client.get_volume(id).await
}

pub fn format_volume_rows(volumes: &[Volumes], long: bool) -> String {
    if long {
        let rows: Vec<VolumeRowLong> = volumes
            .iter()
            .map(|v| VolumeRowLong {
                id: v.id.to_string(),
                name: v.name.clone(),
                size: format!("{} gb", v.size),
                status: format!("{:?}", v.status),
                az: format_opt_ref(&v.availability_zone),
                attached_to: v
                    .attached_server_id
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                volume_type: format_opt_ref(&v.volume_type_name),
                bootable: if v.bootable { "yes" } else { "no" }.to_string(),
                device: format_opt_ref(&v.device),
                created_at: format_date(&v.created_at),
            })
            .collect();
        format_table(&rows)
    } else {
        let rows: Vec<VolumeRow> = volumes
            .iter()
            .map(|v| VolumeRow {
                id: v.id.to_string(),
                name: v.name.clone(),
                size: format!("{} gb", v.size),
                status: format!("{:?}", v.status),
                az: format_opt_ref(&v.availability_zone),
            })
            .collect();
        format_table(&rows)
    }
}

pub fn format_volume_show_row(volume: &Volumes) -> String {
    let row = VolumeShowRow {
        id: volume.id.to_string(),
        name: volume.name.clone(),
        size: format!("{} gb", volume.size),
        status: format!("{:?}", volume.status),
        volume_type: format_opt_ref(&volume.volume_type_name),
        az: format_opt_ref(&volume.availability_zone),
        bootable: if volume.bootable { "yes" } else { "no" }.to_string(),
        attached_server_id: volume
            .attached_server_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "-".to_string()),
        device: format_opt_ref(&volume.device),
        created_at: format_date(&volume.created_at),
        updated_at: format_date(&volume.updated_at),
    };
    format_table(&[row])
}

pub fn format_snapshot_rows(snapshots: &[Snapshots]) -> String {
    let rows: Vec<SnapshotRow> = snapshots
        .iter()
        .map(|s| SnapshotRow {
            id: s.id.to_string(),
            name: s.name.clone(),
            size: format!("{} GB", s.size),
            status: s.status.clone(),
            volume: format_opt(s.volume_name.clone().or(s.volume_type_name.clone())),
            created: format_date(&s.created_at),
        })
        .collect();
    format_table(&rows)
}

pub async fn list_backups(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<Backups>, ComputeError> {
    client.list_backups(limit, page).await
}

pub fn format_backup_rows(backups: &[Backups], long: bool) -> String {
    if long {
        let rows: Vec<BackupRowLong> = backups
            .iter()
            .map(|b| BackupRowLong {
                id: b.id.to_string(),
                name: b.source_object_name.clone(),
                type_: b.backup_type.clone(),
                state: b.state.clone(),
                order_id: b.order_id.to_string(),
                last_backup: format_opt(b.last_backup_time.clone()),
                size_gb: format!("{:.2}", b.size_gb),
                availability_zone: b.availability_zone_name.clone(),
                provider: b.provider.clone(),
                created: format_date(&b.created_time),
            })
            .collect();
        format_table(&rows)
    } else {
        let rows: Vec<BackupRow> = backups
            .iter()
            .map(|b| BackupRow {
                id: b.id.to_string(),
                name: b.source_object_name.clone(),
                type_: b.backup_type.clone(),
                state: b.state.clone(),
                last_backup: format_opt(b.last_backup_time.clone()),
                size_gb: format!("{:.2}", b.size_gb),
                availability_zone: b.availability_zone_name.clone(),
            })
            .collect();
        format_table(&rows)
    }
}
