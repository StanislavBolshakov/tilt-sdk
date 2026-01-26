use anyhow::{Context, Result};
use clap::Parser;
use std::time::Duration;

mod cli;
mod client;
mod completion;
mod compute;
mod output;
mod portal;
mod storage;
mod vpc;

use cli::Cli; 
use client::build_client;
use completion::generate_completions;
use tilt_sdk_cloudengine as cloudengine;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.global.debug {
        tracing::Level::DEBUG
    } else if matches!(
        &cli.command,
        cli::Command::Completions { install: true, .. }
    ) {
        tracing::Level::INFO
    } else {
        tracing::Level::WARN
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();

    if let cli::Command::Completions { shell, install } = &cli.command {
        generate_completions(shell, *install);
        return Ok(());
    }

    let client = build_client(
        cli.global.token.clone(),
        cli.global.project.clone(),
        cli.global.url.clone(),
        Some(Duration::from_secs(30)),
    )
    .context("Failed to build client")?;

    let compute = cloudengine::ComputeClient::new(&client);

    match &cli.command {
        cli::Command::Server { action } => {
            compute::handle_server_action(&compute, action.clone(), None).await
        }
        cli::Command::Flavor { action } => {
            compute::handle_flavor_action(&compute, action.clone(), None).await
        }
        cli::Command::Image { action } => {
            compute::handle_image_action(&compute, action.clone(), None).await
        }
        cli::Command::Task { action } => {
            compute::handle_task_action(&compute, action.clone(), None).await
        }
        cli::Command::Placement { action } => {
            compute::handle_placement_action(&compute, action.clone(), None).await
        }
        cli::Command::Volume { action } => {
            storage::handle_volume_action(&compute, action.clone(), None).await
        }
        cli::Command::Snapshot { action } => {
            storage::handle_snapshot_action(&compute, action.clone(), None).await
        }
        cli::Command::Backup { action } => {
            storage::handle_backup_action(&compute, action.clone(), None).await
        }
        cli::Command::Network { action } => {
            vpc::handle_network_action(&compute, action.clone(), None).await
        }
        cli::Command::Subnet { action } => {
            vpc::handle_subnet_action(&compute, action.clone(), None).await
        }
        cli::Command::Port { action } => {
            vpc::handle_port_action(&compute, action.clone(), None).await
        }
        cli::Command::SecurityGroup { action } => {
            vpc::handle_security_group_action(&compute, action.clone(), None).await
        }
        cli::Command::SshKey { action } => {
            portal::handle_ssh_key_action(&compute, action.clone(), None).await
        }
        cli::Command::Region { action } => {
            vpc::handle_region_action(&compute, action.clone(), None).await
        }
cli::Command::Az { action } => {
    compute::handle_az_action(&compute, action.clone(), None).await
}
        cli::Command::Router { action } => {
            vpc::handle_router_action(&compute, action.clone(), None).await
        }
        cli::Command::RouteTable { action } => {
            vpc::handle_route_table_action(&compute, action.clone(), None).await
        }
        cli::Command::Vip { action } => {
            vpc::handle_vip_action(&compute, action.clone(), None).await
        }
        cli::Command::Fip { action } => {
            vpc::handle_fip_action(&compute, action.clone(), None).await
        }
        cli::Command::Completions { shell, install } => {
            generate_completions(shell, *install);
        }
    }

    Ok(())
}