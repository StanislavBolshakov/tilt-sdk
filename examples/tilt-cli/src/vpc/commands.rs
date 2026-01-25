use cloudengine::client::PortFilter;
use cloudengine::models::{FloatingIps, NetworkItem, RouteTables, VirtualIps};
use cloudengine::{ComputeError, Networks, SecurityGroupRule, SecurityGroups, SshKeys, Subnets};
use tilt_sdk_cloudengine as cloudengine;

use crate::output::{
    FipRow, FipRowLong, InstanceRow, NetworkItemRow, NetworkRow, NetworkRowLong, NicRow, NetworkRouterRow, NetworkRouterRowLong,
    RouteTableRow, RouterRow, SecurityGroupRow, SecurityGroupRowLong, SecurityGroupRuleRow, SshKeyRow, SubnetRow, SubnetRowLong, VipRow,
    VipRowLong, format_date, format_opt_ref, format_port_tree, format_router_tree, format_table,
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

pub async fn delete_subnet(
    client: &cloudengine::ComputeClient<'_>,
    subnet_id: &str,
) -> Result<serde_json::Value, ComputeError> {
    let id = subnet_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::VpcApi,
            None,
            format!("Invalid subnet ID: {}", subnet_id),
        )
    })?;
    client.delete_subnet(id).await
}

pub async fn delete_network(
    client: &cloudengine::ComputeClient<'_>,
    network_id: &str,
) -> Result<serde_json::Value, ComputeError> {
    let id = network_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::VpcApi,
            None,
            format!("Invalid network ID: {}", network_id),
        )
    })?;
    client.delete_network(id).await
}

<<<<<<< HEAD
pub async fn delete_fip(
    client: &cloudengine::ComputeClient<'_>,
    fip_id: &str,
) -> Result<serde_json::Value, ComputeError> {
    let id = fip_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::VpcApi,
            None,
            format!("Invalid floating IP ID: {}", fip_id),
        )
    })?;
    client.delete_fip(id).await
}

pub async fn delete_vip(
    client: &cloudengine::ComputeClient<'_>,
    vip_id: &str,
) -> Result<serde_json::Value, ComputeError> {
    let id = vip_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::VpcApi,
            None,
            format!("Invalid VIP ID: {}", vip_id),
        )
    })?;
    client.delete_vip(id).await
}

pub async fn delete_security_group(
    client: &cloudengine::ComputeClient<'_>,
    security_group_id: &str,
) -> Result<serde_json::Value, ComputeError> {
    let id = security_group_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::VpcApi,
            None,
            format!("Invalid security group ID: {}", security_group_id),
        )
    })?;
    client.delete_security_group(id).await
}

pub async fn delete_route_table(
    client: &cloudengine::ComputeClient<'_>,
    route_table_id: &str,
) -> Result<serde_json::Value, ComputeError> {
    let id = route_table_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::VpcApi,
            None,
            format!("Invalid route table ID: {}", route_table_id),
        )
    })?;
    client.delete_route_table(id).await
}

=======
>>>>>>> 617f703e01ec429df6fdc874184eb8915ea70dc0
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

pub async fn delete_port(
    client: &cloudengine::ComputeClient<'_>,
    port_id: &str,
) -> Result<serde_json::Value, ComputeError> {
    let id = port_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::VpcApi,
            None,
            format!("Invalid port ID: {}", port_id),
        )
    })?;
    client.delete_port(id).await
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

pub fn format_network_rows(networks: &[Networks], long: bool) -> String {
    if long {
        let rows: Vec<NetworkRowLong> = networks
            .iter()
            .map(|n| {
                let dhcp = n.dhcp_settings.as_ref().map(|d| {
                    let dns = format!(
                        "{}/{}",
                        d.dns.method,
                        if d.dns.reverse_resolution { "reverse" } else { "no-reverse" }
                    );
                    let ntp = if d.ntp_servers.is_empty() {
                        String::from("-")
                    } else {
                        d.ntp_servers.join(", ")
                    };
                    format!(
                        "enabled\nDomain: {}\nDNS: {}\nNTP: {}",
                        d.domain_name.clone().unwrap_or_else(|| String::from("-")),
                        dns,
                        ntp
                    )
                }).unwrap_or_else(|| String::from("disabled"));

                let shared = format_opt_ref(&n.shared_from);

                let created = n.create_time.as_ref().map(|t| format_date(t)).unwrap_or_else(|| "-".to_string());

                NetworkRowLong {
                    id: n.id.to_string(),
                    name: n.name.clone(),
                    status: n.status.clone(),
                    dhcp,
                    description: format_opt_ref(&n.description),
                    shared,
                    created,
                }
            })
            .collect();
        format_table(&rows)
    } else {
        let rows: Vec<NetworkRow> = networks
            .iter()
            .map(|n| NetworkRow {
                id: n.id.to_string(),
                name: n.name.clone(),
                status: n.status.clone(),
                description: format_opt_ref(&n.description),
                created: n.create_time.as_ref().map(|t| format_date(t)).unwrap_or_else(|| "-".to_string()),
            })
            .collect();
        format_table(&rows)
    }
}

pub fn format_subnet_rows(subnets: &[Subnets], long: bool) -> String {
    if long {
        let rows: Vec<SubnetRowLong> = subnets
            .iter()
            .map(|s| {
                let from = s.shared_from.clone();
                let to: String = if s.shared_to.is_empty() {
                    String::new()
                } else {
                    format!("To: {}", s.shared_to.join(", "))
                };
                let shared = match (from, to.is_empty()) {
                    (Some(f), true) => format!("From: {}", f),
                    (Some(f), false) => format!("From: {}, {}", f, to),
                    (None, false) => to,
                    (None, true) => String::from("-"),
                };
                let dhcp = if s.enable_dhcp {
                    let settings = s.dhcp_settings.as_ref();
                    let domain = settings.and_then(|d| d.domain_name.clone()).unwrap_or_else(|| String::from("-"));
                    let dns = settings.map(|d| format!(
                        "{}/{}",
                        d.dns.method,
                        if d.dns.reverse_resolution { "reverse" } else { "no-reverse" }
                    )).unwrap_or_else(|| String::from("-"));
                    let ntp = settings
                        .map(|d| d.ntp_servers.join(", "))
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| String::from("-"));
                    format!("enabled\n  Domain: {}\n  DNS: {}\n  NTP: {}", domain, dns, ntp)
                } else {
                    String::from("disabled")
                };
                SubnetRowLong {
                    id: format!(
                        "{}\n└── network_id: {}\n└── region_id: {}",
                        s.id,
                        s.network_id,
                        s.region_id.clone()
                    ),
                    name: s.name.clone(),
                    ipam: format!(
                        "CIDR: {}\nGateway: {}",
                        s.cidr,
                        format_opt_ref(&s.gateway_ip)
                    ),
                    network: s.network_name.clone(),
                    region: s.region_name.clone(),
                    status: s.status.clone(),
                    dhcp,
                    description: format_opt_ref(&s.description),
                    shared,
                }
            })
            .collect();
        format_table(&rows)
    } else {
        let rows: Vec<SubnetRow> = subnets
            .iter()
            .map(|s| SubnetRow {
                id: s.id.to_string(),
                name: s.name.clone(),
                ipam: format!(
                    "{} / {}",
                    s.cidr,
                    format_opt_ref(&s.gateway_ip)
                ),
                network: s.network_name.clone(),
                status: s.status.clone(),
            })
            .collect();
        format_table(&rows)
    }
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
                        ip: format_opt_ref(&nic.ip_address),
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
                        ip: format_opt_ref(&inst.ip_address),
                        network: format_opt_ref(&inst.network_name),
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
                description: format_opt_ref(&g.description),
                created: format_date(&g.create_time),
                updated: g.update_time.as_ref().map(|t| format_date(t)).unwrap_or_else(|| "-".to_string()),
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
                description: format_opt_ref(&g.description),
                created: format_date(&g.create_time),
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

pub enum RoutersListResult {
    Snat(Vec<cloudengine::models::Routers>),
    Network(Vec<cloudengine::models::NetworkRouter>),
}

pub async fn list_routers(
    client: &cloudengine::ComputeClient<'_>,
    r#type: crate::vpc::RouterType,
) -> Result<RoutersListResult, ComputeError> {
    match r#type {
        crate::vpc::RouterType::Snat => {
            let snat_routers = client.list_routers().await?;
            Ok(RoutersListResult::Snat(snat_routers))
        }
        crate::vpc::RouterType::Network => {
            let network_routers = client.list_network_routers(None, None).await?;
            Ok(RoutersListResult::Network(network_routers))
        }
    }
}

pub async fn delete_router(
    client: &cloudengine::ComputeClient<'_>,
    router_id: &str,
    r#type: crate::vpc::RouterType,
) -> Result<serde_json::Value, ComputeError> {
    let id = router_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::VpcApi,
            None,
            format!("Invalid router ID: {}", router_id),
        )
    })?;

    match r#type {
        crate::vpc::RouterType::Snat => {
            client.delete_router(id).await
        }
        crate::vpc::RouterType::Network => {
            client.delete_network_router(id).await
        }
    }
}

pub fn format_router_rows(
    result: &RoutersListResult,
    long: bool,
) -> String {
    match result {
        RoutersListResult::Snat(snat_routers) => {
            if long {
                format_router_tree(snat_routers)
            } else {
                let rows: Vec<RouterRow> = snat_routers
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
        RoutersListResult::Network(network_routers) => {
            if long {
                let rows: Vec<NetworkRouterRowLong> = network_routers
                    .iter()
                    .map(|r| NetworkRouterRowLong {
                        id: r.id.to_string(),
                        name: r.name.clone(),
                        status: r.status.clone(),
                        description: r.description.clone().unwrap_or_else(|| "-".to_string()),
                        created: format_date(&r.create_time),
                        updated: r.update_time.as_ref().map(|t| format_date(t)).unwrap_or_else(|| "-".to_string()),
                    })
                    .collect();
                format_table(&rows)
            } else {
                let rows: Vec<NetworkRouterRow> = network_routers
                    .iter()
                    .map(|r| NetworkRouterRow {
                        id: r.id.to_string(),
                        name: r.name.clone(),
                        status: r.status.clone(),
                        description: r.description.clone().unwrap_or_else(|| "-".to_string()),
                        created: format_date(&r.create_time),
                    })
                    .collect();
                format_table(&rows)
            }
        }
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
                created: format_date(&t.create_date),
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
            let remote_ip = format_opt_ref(&r.remote_ip_prefix);
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

pub fn format_vip_rows(vips: &[VirtualIps], long: bool) -> String {
    if long {
        let rows: Vec<VipRowLong> = vips
            .iter()
            .map(|v| {
                let created = v.created_time.as_ref().map(|t| format_date(t)).unwrap_or_else(|| "-".to_string());

                let floating_ip = v
                    .floating_ip
                    .as_ref()
                    .map(|f| format!("{} ({} Mbps)", f.ip_address, f.bandwidth))
                    .unwrap_or_else(|| "-".to_string());

                let fixed_ip = v
                    .fixed_ips
                    .first()
                    .map(|ip| ip.ip_address.clone())
                    .unwrap_or_else(|| "-".to_string());

                VipRowLong {
                    id: format!(
                        "{}\n└── subnet: {} ({})\n└── network: {} ({})\n└── region: {} ({})",
                        v.id,
                        format_opt_ref(&v.subnet_name),
                        format_opt_ref(&v.subnet_id),
                        format_opt_ref(&v.network_name),
                        format_opt_ref(&v.network_id),
                        format_opt_ref(&v.region_name),
                        format_opt_ref(&v.region_id)
                    ),
                    name: v.name.clone(),
                    status: v.status.clone(),
                    mac_address: v.mac_address.clone(),
                    floating_ip,
                    fixed_ip,
                    l2_enabled: if v.l2_enabled {
                        "enabled".to_string()
                    } else {
                        "disabled".to_string()
                    },
                    address_mode: v.address_mode.clone(),
                    created,
                }
            })
            .collect();
        format_table(&rows)
    } else {
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
}

pub async fn list_floating_ips(
    client: &cloudengine::ComputeClient<'_>,
) -> Result<Vec<FloatingIps>, ComputeError> {
    client.list_floating_ips().await
}

pub fn format_fip_rows(fips: &[FloatingIps], long: bool) -> String {
    if long {
        let rows: Vec<FipRowLong> = fips
            .iter()
            .map(|f| {
                let created = f.created_time.as_ref().map(|t| format_date(t)).unwrap_or_else(|| "-".to_string());

                let parent = f
                    .parent_item_id
                    .as_ref()
                    .map(|pid| {
                        let ptype = format_opt_ref(&f.parent_item_type);
                        format!("{} ({})", ptype, pid)
                    })
                    .unwrap_or_else(|| "-".to_string());

                FipRowLong {
                    id: f.id.to_string(),
                    ip_address: f.floating_ip_address.clone(),
                    bandwidth: format!("{} Mbps", f.bandwidth),
                    status: f.status.clone(),
                    parent,
                    created,
                }
            })
            .collect();
        format_table(&rows)
    } else {
        let rows: Vec<FipRow> = fips
            .iter()
            .map(|f| FipRow {
                id: f.id.to_string(),
                ip_address: f.floating_ip_address.clone(),
                bandwidth: format!("{} Mbps", f.bandwidth),
                status: f.status.clone(),
                created: f.created_time.as_ref().map(|t| format_date(t)).unwrap_or_else(|| "-".to_string()),
            })
            .collect();
        format_table(&rows)
    }
}
