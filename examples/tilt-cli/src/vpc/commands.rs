use cloudengine::client::PortFilter;
use cloudengine::models::{FloatingIps, NetworkItem, RouteTables, Routers, VirtualIps};
use cloudengine::{ComputeError, Networks, SecurityGroupRule, SecurityGroups, SshKeys, Subnets};
use tilt_sdk_cloudengine as cloudengine;

use crate::output::{
    FipRow, InstanceRow, NetworkItemRow, NetworkRow, NicRow, RouteTableRow, RouterRow,
    SecurityGroupRow, SecurityGroupRowLong, SecurityGroupRuleRow, SshKeyRow, SubnetRow, VipRow,
    format_port_tree, format_router_tree, format_table,
};

pub async fn list_networks(
    client: &cloudengine::ComputeClient<'_>,
) -> Result<Vec<Networks>, ComputeError> {
    client.list_networks().await
}

pub async fn list_subnets(
    client: &cloudengine::ComputeClient<'_>,
    network_id: Option<&str>,
) -> Result<Vec<Subnets>, ComputeError> {
    client.list_subnets(network_id).await
}

pub async fn list_ports(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
    filter: PortFilter,
) -> Result<Vec<NetworkItem>, ComputeError> {
    let (limit, page) = match filter {
        PortFilter::Instance => (None, None),
        _ => (limit, page),
    };
    client.list_ports(limit, page, filter).await
}

pub async fn list_security_groups(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<SecurityGroups>, ComputeError> {
    client.list_security_groups(limit, page).await
}

pub async fn list_ssh_keys(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<SshKeys>, ComputeError> {
    client.list_ssh_keys(limit, page).await
}

pub fn format_network_rows(networks: &[Networks]) -> String {
    let rows: Vec<NetworkRow> = networks
        .iter()
        .map(|n| NetworkRow {
            id: n.id.to_string(),
            name: n.name.clone(),
            status: n.status.clone(),
            description: n.description.clone().unwrap_or_else(|| "-".to_string()),
            created: n
                .create_time
                .as_ref()
                .and_then(|t| t.split('T').next())
                .unwrap_or("-")
                .to_string(),
        })
        .collect();
    format_table(&rows)
}

pub fn format_subnet_rows(subnets: &[Subnets]) -> String {
    let rows: Vec<SubnetRow> = subnets
        .iter()
        .map(|s| SubnetRow {
            id: s.id.to_string(),
            name: s.name.clone(),
            cidr: s.cidr.clone(),
            gateway_ip: s.gateway_ip.clone().unwrap_or_else(|| "-".to_string()),
            network: s.network_name.clone(),
            status: s.status.clone(),
        })
        .collect();
    format_table(&rows)
}

pub fn format_port_rows(items: &[NetworkItem], long: bool, filter: PortFilter) -> String {
    if long {
        return format_port_tree(items);
    }

    match filter {
        PortFilter::Nic => {
            let rows: Vec<NicRow> = items
                .iter()
                .filter_map(|item| match item {
                    NetworkItem::Nic(nic) => Some(NicRow {
                        id: nic.id.to_string(),
                        status: nic.state.clone(),
                        ip: nic.ip_address.clone().unwrap_or_else(|| "-".to_string()),
                        network: nic.network_name.clone(),
                        security_groups: if nic.security_group_names.is_empty() {
                            "-".to_string()
                        } else {
                            nic.security_group_names.join(", ")
                        },
                        state: nic.state.clone(),
                    }),
                    NetworkItem::Instance(_) => None,
                })
                .collect();
            format_table(&rows)
        }
        PortFilter::Instance => {
            let rows: Vec<InstanceRow> = items
                .iter()
                .filter_map(|item| match item {
                    NetworkItem::Instance(inst) => Some(InstanceRow {
                        id: inst.id.to_string(),
                        name: inst.name.clone(),
                        status: inst.status.clone(),
                        flavor: inst.flavor_name.clone(),
                        image: inst.image_name.clone(),
                        ip: inst.ip_address.clone().unwrap_or_else(|| "-".to_string()),
                        network: inst.network_name.clone().unwrap_or_else(|| "-".to_string()),
                        availability_zone: inst.availability_zone.clone(),
                    }),
                    NetworkItem::Nic(_) => None,
                })
                .collect();
            format_table(&rows)
        }
        PortFilter::All => {
            let rows: Vec<NetworkItemRow> = items.iter().map(NetworkItemRow::from).collect();
            format_table(&rows)
        }
    }
}

pub fn format_security_group_rows(groups: &[SecurityGroups], long: bool) -> String {
    if long {
        let rows: Vec<SecurityGroupRowLong> = groups
            .iter()
            .map(|g| SecurityGroupRowLong {
                id: g.id.to_string(),
                name: g.name.clone(),
                status: g.status.clone(),
                description: g.description.clone().unwrap_or_else(|| "-".to_string()),
                created: g.create_time.split('T').next().unwrap_or("-").to_string(),
                updated: g
                    .update_time
                    .as_ref()
                    .map(|t| t.split('T').next().unwrap_or("-"))
                    .unwrap_or("-")
                    .to_string(),
            })
            .collect();
        format_table(&rows)
    } else {
        let rows: Vec<SecurityGroupRow> = groups
            .iter()
            .map(|g| SecurityGroupRow {
                id: g.id.to_string(),
                name: g.name.clone(),
                status: g.status.clone(),
                description: g.description.clone().unwrap_or_else(|| "-".to_string()),
                created: g.create_time.split('T').next().unwrap_or("-").to_string(),
            })
            .collect();
        format_table(&rows)
    }
}

pub fn format_ssh_key_rows(keys: &[SshKeys]) -> String {
    let rows: Vec<SshKeyRow> = keys
        .iter()
        .map(|k| SshKeyRow {
            id: k.id.to_string(),
            name: k.name.clone(),
            login: k.login.clone(),
            created: k.created_at.format("%Y-%m-%d").to_string(),
        })
        .collect();
    format_table(&rows)
}

pub async fn list_routers(
    client: &cloudengine::ComputeClient<'_>,
) -> Result<Vec<Routers>, ComputeError> {
    client.list_routers().await
}

pub fn format_router_rows(routers: &[Routers], long: bool) -> String {
    if long {
        format_router_tree(routers)
    } else {
        let rows: Vec<RouterRow> = routers
            .iter()
            .map(|r| RouterRow {
                id: r.id.to_string(),
                name: r.name.clone(),
                status: r.status.to_string(),
                bandwidth: format!("{} Mbps", r.bandwidth),
                ip: r.ip_address.clone(),
            })
            .collect();
        format_table(&rows)
    }
}

pub async fn list_route_tables(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<RouteTables>, ComputeError> {
    client.list_route_tables(limit, page).await
}

pub fn format_route_table_rows(tables: &[RouteTables]) -> String {
    let rows: Vec<RouteTableRow> = tables
        .iter()
        .map(|t| RouteTableRow {
            id: t.id.to_string(),
            name: t.name.clone(),
            status: t.status.to_string(),
            routes: t.routes.len().to_string(),
            networks: t
                .networks
                .iter()
                .map(|n| n.name.clone())
                .collect::<Vec<_>>()
                .join(", "),
            region: t.region.name.clone(),
            created: t.create_date.split('T').next().unwrap_or("-").to_string(),
        })
        .collect();
    format_table(&rows)
}

pub async fn show_security_group_rules(
    client: &cloudengine::ComputeClient<'_>,
    security_group_id: &str,
) -> Result<Vec<SecurityGroupRule>, ComputeError> {
    let id = security_group_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::VpcApi,
            None,
            format!("Invalid security group ID: {}", security_group_id),
        )
    })?;
    client.list_security_group_rules(id).await
}

pub fn format_security_group_rules(rules: &[SecurityGroupRule]) -> String {
    let rows: Vec<SecurityGroupRuleRow> = rules
        .iter()
        .map(|r| {
            let ports = match (r.port_range_min, r.port_range_max) {
                (Some(min), Some(max)) if min == max => min.to_string(),
                (Some(min), Some(max)) => format!("{}-{}", min, max),
                _ => "any".to_string(),
            };
            let remote_ip = r
                .remote_ip_prefix
                .clone()
                .unwrap_or_else(|| "-".to_string());
            SecurityGroupRuleRow {
                id: r.id.to_string(),
                direction: r.direction.clone(),
                protocol: r.protocol.clone(),
                ports,
                remote_ip,
                ethertype: r.ethertype.clone(),
            }
        })
        .collect();
    format_table(&rows)
}

pub async fn list_vips(
    client: &cloudengine::ComputeClient<'_>,
) -> Result<Vec<VirtualIps>, ComputeError> {
    client.list_vips().await
}

pub fn format_vip_rows(vips: &[VirtualIps]) -> String {
    let rows: Vec<VipRow> = vips
        .iter()
        .map(|v| VipRow {
            id: v.id.to_string(),
            name: v.name.clone(),
            status: v.status.clone(),
            floating_ip: v
                .floating_ip
                .as_ref()
                .map(|f| f.ip_address.clone())
                .unwrap_or_else(|| "-".to_string()),
            fixed_ip: v
                .fixed_ips
                .first()
                .map(|ip| ip.ip_address.clone())
                .unwrap_or_else(|| "-".to_string()),
            address_mode: v.address_mode.clone(),
        })
        .collect();
    format_table(&rows)
}

pub async fn list_floating_ips(
    client: &cloudengine::ComputeClient<'_>,
) -> Result<Vec<FloatingIps>, ComputeError> {
    client.list_floating_ips().await
}

pub fn format_fip_rows(fips: &[FloatingIps]) -> String {
    let rows: Vec<FipRow> = fips
        .iter()
        .map(|f| FipRow {
            id: f.id.to_string(),
            ip_address: f.floating_ip_address.clone(),
            bandwidth: format!("{} Mbps", f.bandwidth),
            status: f.status.clone(),
        })
        .collect();
    format_table(&rows)
}
