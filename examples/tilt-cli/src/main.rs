use anyhow::{Context, Result};
use clap::CommandFactory;
use clap::Parser;
use clap_complete::shells::{Bash, Fish, PowerShell, Zsh};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

mod cli;
mod client;
mod compute;
mod output;
mod storage;
mod vpc;

use cli::{Cli, Shell};
use client::build_client;
use tilt_sdk_cloudengine as cloudengine;

fn get_home_dir() -> PathBuf {
    dirs::home_dir().expect("Failed to determine home directory")
}

fn get_pwsh_completion_path() -> PathBuf {
    let mut completion = get_home_dir();
    completion.push("Documents");
    completion.push("WindowsPowerShell");
    completion.push("tilt-cli_completions.ps1");
    completion
}

fn install_completions(shell: &Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    let home_dir = get_home_dir();
    let binary_path = std::env::current_exe()?;
    let _binary_dir = binary_path.parent().unwrap_or_else(|| {
        static DEFAULT: OnceLock<PathBuf> = OnceLock::new();
        DEFAULT.get_or_init(|| PathBuf::from("."))
    });

    match shell {
        Shell::Bash => {
            let completion_path = home_dir.join(".bash_completion.d").join("tilt-cli.bash");
            if let Some(parent) = completion_path.parent()
                && !parent.exists()
            {
                fs::create_dir_all(parent)?;
            }
            let mut file = fs::File::create(&completion_path)?;
            clap_complete::generate(Bash, &mut cmd, bin_name.clone(), &mut file);
            tracing::info!("Installed completion to: {}", completion_path.display());
            println!("Please restart your shell or run: source ~/.bashrc");
        }
        Shell::Zsh => {
            let completion_dir = home_dir.join(".zsh").join("completion");
            if !completion_dir.exists() {
                fs::create_dir_all(&completion_dir)?;
            }
            let completion_path = completion_dir.join("_tilt-cli");
            let mut file = fs::File::create(&completion_path)?;
            clap_complete::generate(Zsh, &mut cmd, bin_name.clone(), &mut file);
            tracing::info!("Installed completion to: {}", completion_path.display());
            println!("Please restart your shell or run: autoload -Uz compinit && compinit");
        }
        Shell::Fish => {
            let completion_dir = home_dir.join(".config").join("fish").join("completions");
            if !completion_dir.exists() {
                fs::create_dir_all(&completion_dir)?;
            }
            let completion_path = completion_dir.join("tilt-cli.fish");
            let mut file = fs::File::create(&completion_path)?;
            clap_complete::generate(Fish, &mut cmd, bin_name.clone(), &mut file);
            tracing::info!("Installed completion to: {}", completion_path.display());
            println!(
                "Please restart your shell or run: fish -c 'source ~/.config/fish/config.fish'"
            );
        }
        Shell::Powershell => {
            let completion_path = get_pwsh_completion_path();
            let mut file = fs::File::create(&completion_path)?;
            clap_complete::generate(PowerShell, &mut cmd, bin_name.clone(), &mut file);
            tracing::info!("Installed completion to: {}", completion_path.display());
            println!("Please restart PowerShell or run: . $PROFILE");
        }
    }
    Ok(())
}

fn generate_completions(shell: &Shell, install: bool) {
    if install {
        if let Err(e) = install_completions(shell) {
            tracing::error!("Failed to install completions: {}", e);
        }
        return;
    }
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    match shell {
        Shell::Bash => clap_complete::generate(Bash, &mut cmd, bin_name, &mut std::io::stdout()),
        Shell::Zsh => clap_complete::generate(Zsh, &mut cmd, bin_name, &mut std::io::stdout()),
        Shell::Fish => clap_complete::generate(Fish, &mut cmd, bin_name, &mut std::io::stdout()),
        Shell::Powershell => {
            clap_complete::generate(PowerShell, &mut cmd, bin_name, &mut std::io::stdout())
        }
    }
}

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
            vpc::handle_ssh_key_action(&compute, action.clone(), None).await
        }
        cli::Command::Region { action } => {
            vpc::handle_region_action(&compute, action.clone(), None).await
        }
        cli::Command::Az { action } => {
            vpc::handle_region_action(&compute, action.clone(), None).await
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
