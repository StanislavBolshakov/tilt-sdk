use cloudengine::{
    ComputeError, Flavors, Images, Instances, PlacementPolicy, Regions, Service, Tasks,
};
use tilt_sdk_cloudengine as cloudengine;

use crate::output::{
    FlavorsRow, ImageRow, ImageRowLong, ImageShowRow, PlacementRow, RegionRow, ServersRow,
    ServersRowLong, TaskRow, TaskRowDetail, format_table,
};

pub async fn list_servers(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
    _long: bool,
) -> Result<Vec<Instances>, ComputeError> {
    client.list_instances(limit, page).await
}

pub async fn list_flavors(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
    az: Option<&str>,
) -> Result<Vec<Flavors>, ComputeError> {
    client.list_flavors(limit, page, az).await
}

pub async fn list_project_images(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<Images>, ComputeError> {
    client.list_project_images(limit, page).await
}

pub async fn list_tasks(
    client: &cloudengine::ComputeClient<'_>,
) -> Result<Vec<Tasks>, ComputeError> {
    client.list_tasks().await
}

pub async fn show_task(
    client: &cloudengine::ComputeClient<'_>,
    task_id: &str,
) -> Result<Tasks, ComputeError> {
    let id = task_id
        .parse()
        .map_err(|e| ComputeError::new(Service::Global, None, format!("Invalid task ID: {}", e)))?;
    client.get_task(id).await
}

pub async fn list_regions(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<Regions>, ComputeError> {
    client.list_regions(limit, page).await
}

pub async fn show_image(
    client: &cloudengine::ComputeClient<'_>,
    image_id: &str,
) -> Result<Images, ComputeError> {
    let id = image_id.parse().map_err(|e| {
        ComputeError::new(
            Service::ComputeApi,
            None,
            format!("Invalid image ID: {}", e),
        )
    })?;
    client.get_image(id).await
}

pub fn format_servers_rows(instances: &[Instances], long: bool) -> String {
    if long {
        let rows: Vec<ServersRowLong> = instances
            .iter()
            .map(|i| {
                let ip = if i.all_addresses.is_empty() {
                    "-".to_string()
                } else {
                    i.all_addresses.join(", ")
                };
                ServersRowLong {
                    id: i.id.to_string(),
                    name: i.name.clone(),
                    status: format!("{:?}", i.status),
                    flavor: i.flavor_name.clone().unwrap_or_else(|| "-".to_string()),
                    image: i.image_name.clone().unwrap_or_else(|| "-".to_string()),
                    ip,
                    availability_zone: i
                        .availability_zone
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                    created_at: i.created_row_dt[..10].to_string(),
                }
            })
            .collect();
        format_table(&rows)
    } else {
        let rows: Vec<ServersRow> = instances
            .iter()
            .map(|i| ServersRow {
                id: i.id.to_string(),
                name: i.name.clone(),
                status: format!("{:?}", i.status),
                availability_zone: i
                    .availability_zone
                    .clone()
                    .unwrap_or_else(|| "-".to_string()),
            })
            .collect();
        format_table(&rows)
    }
}

pub fn format_flavors_rows(flavors: &[Flavors]) -> String {
    let rows: Vec<FlavorsRow> = flavors
        .iter()
        .map(|f| FlavorsRow {
            id: f.id.to_string(),
            name: f.name.clone(),
            ram_mb: f.ram_mb.to_string(),
            vcpus: f.vcpus.to_string(),
            gpus: f.gpus.to_string(),
        })
        .collect();
    format_table(&rows)
}

pub fn format_image_rows(images: &[Images], long: bool) -> String {
    if long {
        let rows: Vec<ImageRowLong> = images
            .iter()
            .map(|i| {
                let size_gb = format!("{:.2}", i.size_bytes as f64 / 1_000_000_000.0);
                let created_at = i.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
                let os_distro = i.os_distro.clone().unwrap_or_else(|| "-".to_string());
                let os_version = i.os_version.clone().unwrap_or_else(|| "-".to_string());
                let visibility = i.visibility.clone().unwrap_or_else(|| "-".to_string());
                ImageRowLong {
                    id: i.id.to_string(),
                    name: i.name.clone(),
                    size_gb,
                    status: format!("{:?}", i.status),
                    disk_format: i.disk_format.clone(),
                    os_distro,
                    os_version,
                    visibility,
                    created_at,
                }
            })
            .collect();
        format_table(&rows)
    } else {
        let rows: Vec<ImageRow> = images
            .iter()
            .map(|i| {
                let size_gb = format!("{:.2}", i.size_bytes as f64 / 1_000_000_000.0);
                let created = i.created_at.format("%Y-%m-%d").to_string();
                ImageRow {
                    id: i.id.to_string(),
                    name: i.name.clone(),
                    size_gb,
                    status: format!("{:?}", i.status),
                    created,
                }
            })
            .collect();
        format_table(&rows)
    }
}

pub fn format_image_show_row(image: &Images) -> String {
    let size_gb = format!("{:.2}", image.size_bytes as f64 / 1_000_000_000.0);
    let created_at = image.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
    let updated_at = image
        .updated_at
        .as_ref()
        .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "-".to_string());
    let row = ImageShowRow {
        id: image.id.to_string(),
        name: image.name.clone(),
        size_gb,
        status: format!("{:?}", image.status),
        disk_format: image.disk_format.clone(),
        os_distro: image.os_distro.clone().unwrap_or_else(|| "-".to_string()),
        os_version: image.os_version.clone().unwrap_or_else(|| "-".to_string()),
        visibility: image.visibility.clone().unwrap_or_else(|| "-".to_string()),
        availability_zone: image
            .availability_zone
            .clone()
            .unwrap_or_else(|| "-".to_string()),
        created_at,
        updated_at,
    };
    format_table(&[row])
}

pub fn format_region_rows(regions: &[Regions]) -> String {
    let rows: Vec<RegionRow> = regions
        .iter()
        .map(|r| RegionRow {
            id: r.id.clone(),
            name: r.name.clone(),
            description: r.description.clone().unwrap_or_else(|| "-".to_string()),
        })
        .collect();
    format_table(&rows)
}

pub fn format_task_rows(tasks: &[Tasks], long: bool) -> String {
    if long {
        let rows: Vec<TaskRowDetail> = tasks
            .iter()
            .map(|t| {
                let duration = t
                    .duration_seconds
                    .map(|d| format!("{}s", d))
                    .unwrap_or_else(|| "-".to_string());
                let error = t.error.clone().unwrap_or_else(|| "-".to_string());
                TaskRowDetail {
                    id: t.id.to_string(),
                    object_type: t.object_type.clone(),
                    object_id: t.object_id.to_string(),
                    status: format!("{:?}", t.status),
                    initiator: t.initiator.clone(),
                    created_at: t.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                    duration_s: duration,
                    error,
                }
            })
            .collect();
        format_table(&rows)
    } else {
        let rows: Vec<TaskRow> = tasks
            .iter()
            .map(|t| {
                let duration = t
                    .duration_seconds
                    .map(|d| format!("{}s", d))
                    .unwrap_or_else(|| "-".to_string());
                TaskRow {
                    id: t.id.to_string(),
                    object_type: t.object_type.clone(),
                    object_id: t.object_id.to_string(),
                    status: format!("{:?}", t.status),
                    initiator: t.initiator.clone(),
                    created_at: t.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                    duration_s: duration,
                }
            })
            .collect();
        format_table(&rows)
    }
}

pub async fn list_placement_policies(
    client: &cloudengine::ComputeClient<'_>,
) -> Result<Vec<PlacementPolicy>, ComputeError> {
    client.list_placement_policies().await
}

pub fn format_placement_rows(policies: &[PlacementPolicy], _long: bool) -> String {
    let rows: Vec<PlacementRow> = policies
        .iter()
        .map(|p| PlacementRow {
            id: p.id.to_string(),
            name: p.name.clone(),
            policy_type: p.policy_type.clone().unwrap_or_else(|| "-".to_string()),
            availability_zone: p
                .availability_zone
                .clone()
                .unwrap_or_else(|| "-".to_string()),
        })
        .collect();
    format_table(&rows)
}
