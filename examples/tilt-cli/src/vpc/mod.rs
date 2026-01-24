pub mod commands;

use clap::{Args, Subcommand};

use crate::cli::OutputFormat;
use tilt_sdk_cloudengine as cloudengine;

#[derive(Debug, Clone, Subcommand)]
pub enum NetworkAction {
    #[command(about = "List networks")]
    List {
        #[command(flatten)]
        list_opts: NetworkListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct NetworkListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum SubnetAction {
    #[command(about = "List subnets")]
    List {
        #[command(flatten)]
        list_opts: SubnetListOpts,
    },
    #[command(about = "Delete a subnet")]
    Delete {
        #[command(flatten)]
        delete_opts: SubnetDeleteOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct SubnetListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(short, long, help = "Filter by network ID")]
    pub network_id: Option<String>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
}

#[derive(Debug, Clone, Args)]
pub struct SubnetDeleteOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    pub subnet_id: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum PortAction {
    #[command(about = "List ports")]
    List {
        #[command(flatten)]
        list_opts: PortListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct PortListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
    #[arg(long, help = "Show only NIC ports")]
    pub nic: bool,
    #[arg(long, help = "Show only instance ports")]
    pub instance: bool,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum SecurityGroupAction {
    #[command(about = "List security groups")]
    List {
        #[command(flatten)]
        list_opts: SecurityGroupListOpts,
    },
    #[command(about = "Show security group with rules")]
    Show {
        #[command(flatten)]
        show_opts: SecurityGroupShowOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct SecurityGroupListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Args)]
pub struct SecurityGroupShowOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    pub id: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum SshKeyAction {
    #[command(about = "List SSH keys")]
    List {
        #[command(flatten)]
        list_opts: SshKeyListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct SshKeyListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum RegionAction {
    #[command(about = "List regions")]
    List {
        #[command(flatten)]
        list_opts: RegionListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct RegionListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum RouterAction {
    #[command(about = "List SNAT routers")]
    List {
        #[command(flatten)]
        list_opts: RouterListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct RouterListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum RouteTableAction {
    #[command(about = "List route tables")]
    List {
        #[command(flatten)]
        list_opts: RouteTableListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct RouteTableListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum VipAction {
    #[command(about = "List virtual IPs")]
    List {
        #[command(flatten)]
        list_opts: VipListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct VipListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum FipAction {
    #[command(about = "List floating IPs")]
    List {
        #[command(flatten)]
        list_opts: FipListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct FipListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
}

pub async fn handle_network_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: NetworkAction,
    format: Option<OutputFormat>,
) {
    match action {
        NetworkAction::List { list_opts } => match commands::list_networks(compute).await {
            Ok(networks) => {
                let table = commands::format_network_rows(&networks, list_opts.long);
                match format.unwrap_or(OutputFormat::Table) {
                    OutputFormat::Table => {
                        println!("{}", table);
                        println!(
                            "{}",
                            crate::output::format_count(networks.len(), "network", "networks")
                        );
                    }
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&networks).unwrap());
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "tilt-cli", "{}", e);
                std::process::exit(1);
            }
        },
    }
}

pub async fn handle_subnet_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: SubnetAction,
    format: Option<OutputFormat>,
) {
    match action {
        SubnetAction::List { list_opts } => {
            match commands::list_subnets(compute, list_opts.network_id.as_deref()).await {
                Ok(subnets) => {
                    let table = commands::format_subnet_rows(&subnets, list_opts.long);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(subnets.len(), "subnet", "subnets")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&subnets).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
        SubnetAction::Delete { delete_opts } => {
            match commands::delete_subnet(compute, &delete_opts.subnet_id).await {
                Ok(json) => {
                    match delete_opts.format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("Subnet {} deleted successfully", delete_opts.subnet_id);
                        }
                        OutputFormat::Json => {
                            println!("{}", json);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

pub async fn handle_port_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: PortAction,
    format: Option<OutputFormat>,
) {
    match action {
        PortAction::List { list_opts } => {
            let filter = if list_opts.nic {
                cloudengine::client::PortFilter::Nic
            } else if list_opts.instance {
                cloudengine::client::PortFilter::Instance
            } else {
                cloudengine::client::PortFilter::All
            };
            let filter_for_format = filter.clone();
            let limit = if list_opts.nic || list_opts.instance {
                None
            } else {
                list_opts.limit
            };
            let page = if list_opts.nic || list_opts.instance {
                None
            } else {
                list_opts.page
            };
            match commands::list_ports(compute, limit, page, filter).await {
                Ok(items) => {
                    let table =
                        commands::format_port_rows(&items, list_opts.long, filter_for_format);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(items.len(), "port", "ports")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&items).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

pub async fn handle_security_group_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: SecurityGroupAction,
    format: Option<OutputFormat>,
) {
    match action {
        SecurityGroupAction::List { list_opts } => {
            match commands::list_security_groups(compute, list_opts.limit, list_opts.page).await {
                Ok(groups) => {
                    let table = commands::format_security_group_rows(&groups, list_opts.long);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(
                                    groups.len(),
                                    "security group",
                                    "security groups"
                                )
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&groups).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
        SecurityGroupAction::Show { show_opts } => {
            match commands::show_security_group_rules(compute, &show_opts.id).await {
                Ok(rules) => {
                    let table = commands::format_security_group_rules(&rules);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(rules.len(), "rule", "rules")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&rules).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

pub async fn handle_ssh_key_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: SshKeyAction,
    format: Option<OutputFormat>,
) {
    match action {
        SshKeyAction::List { list_opts } => {
            match commands::list_ssh_keys(compute, list_opts.limit, list_opts.page).await {
                Ok(keys) => {
                    let table = commands::format_ssh_key_rows(&keys);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(keys.len(), "SSH key", "SSH keys")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&keys).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

pub async fn handle_region_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: RegionAction,
    format: Option<OutputFormat>,
) {
    match action {
        RegionAction::List { list_opts } => {
            match crate::compute::commands::list_regions(compute, list_opts.limit, list_opts.page)
                .await
            {
                Ok(regions) => {
                    let table = crate::compute::commands::format_region_rows(&regions);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(regions.len(), "region", "regions")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&regions).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

pub async fn handle_router_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: RouterAction,
    format: Option<OutputFormat>,
) {
    match action {
        RouterAction::List { list_opts } => match commands::list_routers(compute).await {
            Ok(routers) => {
                let table = commands::format_router_rows(&routers, list_opts.long);
                match format.unwrap_or(OutputFormat::Table) {
                    OutputFormat::Table => {
                        println!("{}", table);
                        println!(
                            "{}",
                            crate::output::format_count(routers.len(), "router", "routers")
                        );
                    }
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&routers).unwrap());
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "tilt-cli", "{}", e);
                std::process::exit(1);
            }
        },
    }
}

pub async fn handle_route_table_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: RouteTableAction,
    format: Option<OutputFormat>,
) {
    match action {
        RouteTableAction::List { list_opts } => {
            match commands::list_route_tables(compute, list_opts.limit, list_opts.page).await {
                Ok(tables) => {
                    let table = commands::format_route_table_rows(&tables);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(
                                    tables.len(),
                                    "route table",
                                    "route tables"
                                )
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&tables).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

pub async fn handle_vip_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: VipAction,
    format: Option<OutputFormat>,
) {
    match action {
        VipAction::List { list_opts } => match commands::list_vips(compute).await {
            Ok(vips) => {
                let table = commands::format_vip_rows(&vips, list_opts.long);
                match format.unwrap_or(OutputFormat::Table) {
                    OutputFormat::Table => {
                        println!("{}", table);
                        println!("{}", crate::output::format_count(vips.len(), "VIP", "VIPs"));
                    }
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&vips).unwrap());
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "tilt-cli", "{}", e);
                std::process::exit(1);
            }
        },
    }
}

pub async fn handle_fip_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: FipAction,
    format: Option<OutputFormat>,
) {
    match action {
        FipAction::List { list_opts } => match commands::list_floating_ips(compute).await {
            Ok(fips) => {
                let table = commands::format_fip_rows(&fips, list_opts.long);
                match format.unwrap_or(OutputFormat::Table) {
                    OutputFormat::Table => {
                        println!("{}", table);
                        println!("{}", crate::output::format_count(fips.len(), "FIP", "FIPs"));
                    }
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&fips).unwrap());
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "tilt-cli", "{}", e);
                std::process::exit(1);
            }
        },
    }
}
