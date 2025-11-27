use crate::error::{Result, SuiForgeError};
use std::process::{Command, Output};
use which::which;

pub struct SuiCli;

impl SuiCli {
    #[allow(dead_code)]
    pub fn check_installed() -> Result<()> {
        which("sui").map_err(|_| SuiForgeError::SuiCliNotFound)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn version() -> Result<String> {
        let output = Command::new("sui").arg("--version").output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(SuiForgeError::Custom(
                "Failed to get Sui version".to_string(),
            ))
        }
    }

    pub fn build(release: bool) -> Result<Output> {
        let mut cmd = Command::new("sui");
        cmd.arg("move").arg("build");

        if release {
            cmd.arg("--release");
        }

        let output = cmd.output()?;
        Ok(output)
    }

    pub fn test(filter: Option<String>) -> Result<Output> {
        let mut cmd = Command::new("sui");
        cmd.arg("move").arg("test");

        if let Some(f) = filter {
            cmd.arg("--filter").arg(f);
        }

        let output = cmd.output()?;
        Ok(output)
    }

    pub fn publish(network: &str, gas_budget: u64) -> Result<Output> {
        let mut cmd = Command::new("sui");
        cmd.arg("client")
            .arg("publish")
            .arg("--gas-budget")
            .arg(gas_budget.to_string());

        // Add network-specific flags
        match network {
            "devnet" | "testnet" | "mainnet" => {
                cmd.arg("--network").arg(network);
            }
            _ => return Err(SuiForgeError::InvalidNetwork(network.to_string())),
        }

        let output = cmd.output()?;
        Ok(output)
    }

    pub fn get_active_address() -> Result<String> {
        let output = Command::new("sui")
            .arg("client")
            .arg("active-address")
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(SuiForgeError::Custom(
                "Failed to get active address".to_string(),
            ))
        }
    }

    #[allow(dead_code)]
    pub fn get_gas_objects(address: &str) -> Result<Vec<String>> {
        let output = Command::new("sui")
            .arg("client")
            .arg("gas")
            .arg("--address")
            .arg(address)
            .output()?;

        if output.status.success() {
            // Parse gas objects from output
            // This is simplified - actual implementation would parse JSON
            Ok(vec![])
        } else {
            Err(SuiForgeError::Custom(
                "Failed to get gas objects".to_string(),
            ))
        }
    }
}
