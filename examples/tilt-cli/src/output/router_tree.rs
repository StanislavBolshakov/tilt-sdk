use tilt_sdk_cloudengine as cloudengine;

use crate::output::format_date;
use crate::output::format_table;
use crate::output::rows::RouterRowLong;

fn format_router_tree_table(router: &cloudengine::models::Routers) -> Vec<RouterRowLong> {
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

    vec![RouterRowLong {
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
    let mut rows: Vec<RouterRowLong> = Vec::new();

    for router in routers {
        rows.extend(format_router_tree_table(router));
    }

    if rows.is_empty() {
        String::new()
    } else {
        format_table(&rows)
    }
}
