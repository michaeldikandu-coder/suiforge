use crate::error::Result;
use crate::utils;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkProfile {
    pub name: String,
    pub rpc: String,
    pub faucet: Option<String>,
    pub explorer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub active: String,
    pub profiles: HashMap<String, NetworkProfile>,
}

impl ProfileConfig {
    fn config_path() -> PathBuf {
        let home = home::home_dir().expect("Could not find home directory");
        home.join(".suiforge").join("profiles.json")
    }

    fn load() -> Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    fn save(&self) -> Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    fn default() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert(
            "devnet".to_string(),
            NetworkProfile {
                name: "devnet".to_string(),
                rpc: "https://fullnode.devnet.sui.io:443".to_string(),
                faucet: Some("https://faucet.devnet.sui.io/gas".to_string()),
                explorer: Some("https://suiexplorer.com/?network=devnet".to_string()),
            },
        );
        profiles.insert(
            "testnet".to_string(),
            NetworkProfile {
                name: "testnet".to_string(),
                rpc: "https://fullnode.testnet.sui.io:443".to_string(),
                faucet: Some("https://faucet.testnet.sui.io/gas".to_string()),
                explorer: Some("https://suiexplorer.com/?network=testnet".to_string()),
            },
        );
        profiles.insert(
            "mainnet".to_string(),
            NetworkProfile {
                name: "mainnet".to_string(),
                rpc: "https://fullnode.mainnet.sui.io:443".to_string(),
                faucet: None,
                explorer: Some("https://suiexplorer.com/?network=mainnet".to_string()),
            },
        );

        Self {
            active: "devnet".to_string(),
            profiles,
        }
    }
}

pub async fn execute(action: String, name: Option<String>, rpc: Option<String>) -> Result<()> {
    match action.as_str() {
        "list" => list_profiles().await,
        "add" => add_profile(name, rpc).await,
        "remove" => remove_profile(name).await,
        "switch" => switch_profile(name).await,
        "show" => show_profile(name).await,
        _ => {
            utils::error(&format!("Unknown action: {}", action));
            println!("Available actions: list, add, remove, switch, show");
            Ok(())
        }
    }
}

async fn list_profiles() -> Result<()> {
    let config = ProfileConfig::load()?;

    utils::info("Network Profiles:");
    println!();

    for (name, profile) in &config.profiles {
        let active_marker = if name == &config.active {
            "●".green().bold()
        } else {
            "○".dimmed()
        };

        println!("{} {} {}", active_marker, name.cyan().bold(), profile.rpc.dimmed());
        if let Some(faucet) = &profile.faucet {
            println!("    Faucet: {}", faucet.dimmed());
        }
    }

    println!();
    utils::info(&format!("Active profile: {}", config.active.green().bold()));

    Ok(())
}

async fn add_profile(name: Option<String>, rpc: Option<String>) -> Result<()> {
    let name = name.expect("Profile name is required");
    let rpc = rpc.expect("RPC URL is required");

    let mut config = ProfileConfig::load()?;

    config.profiles.insert(
        name.clone(),
        NetworkProfile {
            name: name.clone(),
            rpc: rpc.clone(),
            faucet: None,
            explorer: None,
        },
    );

    config.save()?;

    utils::success(&format!("Added profile: {} → {}", name.green(), rpc));

    Ok(())
}

async fn remove_profile(name: Option<String>) -> Result<()> {
    let name = name.expect("Profile name is required");

    let mut config = ProfileConfig::load()?;

    if config.profiles.remove(&name).is_some() {
        config.save()?;
        utils::success(&format!("Removed profile: {}", name.red()));
    } else {
        utils::error(&format!("Profile not found: {}", name));
    }

    Ok(())
}

async fn switch_profile(name: Option<String>) -> Result<()> {
    let name = name.expect("Profile name is required");

    let mut config = ProfileConfig::load()?;

    if config.profiles.contains_key(&name) {
        config.active = name.clone();
        config.save()?;
        utils::success(&format!("Switched to profile: {}", name.green().bold()));
    } else {
        utils::error(&format!("Profile not found: {}", name));
    }

    Ok(())
}

async fn show_profile(name: Option<String>) -> Result<()> {
    let config = ProfileConfig::load()?;

    let profile_name = name.unwrap_or(config.active.clone());

    if let Some(profile) = config.profiles.get(&profile_name) {
        println!();
        println!("{}", "Profile Details:".bold());
        println!("  Name: {}", profile.name.cyan());
        println!("  RPC: {}", profile.rpc.yellow());
        if let Some(faucet) = &profile.faucet {
            println!("  Faucet: {}", faucet.dimmed());
        }
        if let Some(explorer) = &profile.explorer {
            println!("  Explorer: {}", explorer.dimmed());
        }
        println!();
    } else {
        utils::error(&format!("Profile not found: {}", profile_name));
    }

    Ok(())
}
