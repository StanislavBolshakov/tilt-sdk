use anyhow::Result;
use clap::CommandFactory;
use clap_complete::shells::{Bash, Fish, PowerShell, Zsh};
use std::fs;
use std::path::PathBuf;

use crate::cli::Shell;

pub fn get_home_dir() -> PathBuf {
    dirs::home_dir().expect("Failed to determine home directory")
}

pub fn get_pwsh_completion_path() -> PathBuf {
    let mut completion = get_home_dir();
    completion.push("Documents");
    completion.push("WindowsPowerShell");
    completion.push("tilt-cli_completions.ps1");
    completion
}

pub fn install_completions(shell: &Shell) -> Result<()> {
    let mut cmd = crate::cli::Cli::command();
    let bin_name = cmd.get_name().to_string();
    let home_dir = get_home_dir();

    match shell {
        Shell::Bash => {
            let completion_path = home_dir.join(".bash_completion.d").join("tilt-cli.bash");
            if let Some(parent) = completion_path.parent()
                && !parent.exists() {
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

pub fn generate_completions(shell: &Shell, install: bool) {
    if install {
        if let Err(e) = install_completions(shell) {
            tracing::error!("Failed to install completions: {}", e);
        }
        return;
    }
    let mut cmd = crate::cli::Cli::command();
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