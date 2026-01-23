pub mod common;
pub mod rows;

pub use common::format_bytes;
pub use common::format_count;
pub use common::format_date;
pub use common::format_opt;
pub use common::format_opt_ref;
pub use rows::*;

use comfy_table::Table;
use tilt_sdk_cloudengine as cloudengine;

pub fn format_table<T: ToStringRow>(rows: &[T]) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let mut table = Table::new();
    table.set_style(comfy_table::TableComponent::VerticalLines, ' ');

    // Add headers from the first row
    let headers = rows[0].to_headers();
    table.set_header(headers);

    // Add data rows
    for row in rows {
        table.add_row(row.to_row());
    }

    table.to_string()
}

fn format_instance_tree_table(
    inst: &cloudengine::models::InstanceItem,
    nics: &[cloudengine::models::NicPort],
) -> Vec<rows::PortTreeRow> {
    let mut rows = Vec::new();

    let instance_id = inst.id.to_string();
    let network = format_opt_ref(&inst.network_name);

    if nics.is_empty() {
        rows.push(rows::PortTreeRow {
            instance: inst.name.clone(),
            instance_id,
            network,
            instance_status: inst.status.clone(),
            az: inst.availability_zone.clone(),
            attached_nics: String::new(),
        });
    } else {
        for (i, nic) in nics.iter().enumerate() {
            let is_last = i == nics.len() - 1;
            let prefix = if is_last { "└── " } else { "├── " };
            let details_indent = "     ";

            let nic_ip = nic.ip_address.clone().unwrap_or_else(|| "-".to_string());
            let sgs = if nic.security_group_names.is_empty() {
                String::new()
            } else {
                format!(
                    "\n{}sg: {}",
                    details_indent,
                    nic.security_group_names.join(", ")
                )
            };

            let nic_details = format!(
                "{}nic: {}\n{}ip: {}",
                prefix, nic.id, details_indent, nic_ip
            );

            rows.push(rows::PortTreeRow {
                instance: inst.name.clone(),
                instance_id: instance_id.clone(),
                network: network.clone(),
                instance_status: inst.status.clone(),
                az: inst.availability_zone.clone(),
                attached_nics: format!("{}{}", nic_details, sgs),
            });
        }
    }

    rows
}

fn format_standalone_nic_row(nic: &cloudengine::models::NicPort) -> rows::PortTreeRow {
    let nic_ip = format_opt_ref(&nic.ip_address);
    let sgs = if nic.security_group_names.is_empty() {
        String::new()
    } else {
        format!("\n     sg: {}", nic.security_group_names.join(", "))
    };

    rows::PortTreeRow {
        instance: "-".to_string(),
        instance_id: "-".to_string(),
        network: nic.network_name.clone(),
        az: "-".to_string(),
        instance_status: "-".to_string(),
        attached_nics: format!("└── nic: {}\n     ip: {}{}", nic.id, nic_ip, sgs),
    }
}

pub fn format_port_tree(items: &[cloudengine::models::NetworkItem]) -> String {
    use std::collections::HashMap;

    let mut instances: HashMap<
        String,
        (
            cloudengine::models::InstanceItem,
            Vec<cloudengine::models::NicPort>,
        ),
    > = HashMap::new();
    let mut standalone_nics: Vec<cloudengine::models::NicPort> = Vec::new();

    for item in items {
        if let cloudengine::models::NetworkItem::Instance(inst) = item {
            instances.insert(inst.id.to_string(), (inst.clone(), Vec::new()));
        }
    }

    for item in items {
        if let cloudengine::models::NetworkItem::Nic(nic) = item {
            if let Some(inst_id) = nic.instance_id {
                let inst_id_str = inst_id.to_string();
                if let Some((_, inst_nics)) = instances.get_mut(&inst_id_str) {
                    inst_nics.push(nic.clone());
                    continue;
                }
            }
            standalone_nics.push(nic.clone());
        }
    }

    let mut rows: Vec<rows::PortTreeRow> = Vec::new();

    for (inst, nics) in instances.values() {
        rows.extend(format_instance_tree_table(inst, nics));
    }

    for nic in &standalone_nics {
        rows.push(format_standalone_nic_row(nic));
    }

    if rows.is_empty() {
        String::new()
    } else {
        format_table(&rows)
    }
}

fn format_router_tree_table(router: &cloudengine::models::Routers) -> Vec<rows::RouterRowLong> {
    let router_id = router.id.to_string();

    let attached_nics = if router.attached_nics.is_empty() {
        String::new()
    } else {
        let mut lines = Vec::new();
        for (i, nic) in router.attached_nics.iter().enumerate() {
            let is_last = i == router.attached_nics.len() - 1;
            let prefix = if is_last { "└── " } else { "├── " };
            let cont_prefix = if is_last { "    " } else { "│   " };

            lines.push(format!("{}nic: {}", prefix, nic.id));
            lines.push(format!("{}ip: {}", cont_prefix, nic.ip_address));
            if let Some(ref network) = nic.network_name {
                lines.push(format!("{}network: {}", cont_prefix, network));
            }
            if let Some(ref subnet) = nic.subnet_name {
                lines.push(format!("{}subnet: {}", cont_prefix, subnet));
            }
        }
        lines.join("\n")
    };

    vec![rows::RouterRowLong {
        id: router_id,
        name: router.name.clone(),
        status: router.status.to_string(),
        bandwidth: format!("{} Mbps", router.bandwidth),
        ip: router.ip_address.clone(),
        attached_nics,
        created_at: format_date(&router.created_at),
    }]
}

pub fn format_router_tree(routers: &[cloudengine::models::Routers]) -> String {
    let mut rows: Vec<rows::RouterRowLong> = Vec::new();

    for router in routers {
        rows.extend(format_router_tree_table(router));
    }

    if rows.is_empty() {
        String::new()
    } else {
        format_table(&rows)
    }
}
