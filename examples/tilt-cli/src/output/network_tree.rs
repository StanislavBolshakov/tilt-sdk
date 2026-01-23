use std::collections::HashMap;

use tilt_sdk_cloudengine as cloudengine;

use crate::output::format_opt_ref;
use crate::output::format_table;
use crate::output::rows::PortTreeRow;

fn format_instance_tree_table(
    inst: &cloudengine::models::InstanceItem,
    nics: &[cloudengine::models::NicPort],
) -> Vec<PortTreeRow> {
    let mut rows = Vec::new();

    let instance_id = inst.id.to_string();
    let network = format_opt_ref(&inst.network_name);

    if nics.is_empty() {
        rows.push(PortTreeRow {
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

            rows.push(PortTreeRow {
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

fn format_standalone_nic_row(nic: &cloudengine::models::NicPort) -> PortTreeRow {
    let nic_ip = format_opt_ref(&nic.ip_address);
    let sgs = if nic.security_group_names.is_empty() {
        String::new()
    } else {
        format!("\n     sg: {}", nic.security_group_names.join(", "))
    };

    PortTreeRow {
        instance: "-".to_string(),
        instance_id: "-".to_string(),
        network: nic.network_name.clone(),
        az: "-".to_string(),
        instance_status: "-".to_string(),
        attached_nics: format!("└── nic: {}\n     ip: {}{}", nic.id, nic_ip, sgs),
    }
}

pub fn format_port_tree(items: &[cloudengine::models::NetworkItem]) -> String {
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

    let mut rows: Vec<PortTreeRow> = Vec::new();

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
