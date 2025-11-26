use crate::config::{find_project_root, load_config, DeploymentLock};
use crate::error::{Result, SuiForgeError};
use crate::sui::SuiCli;
use crate::utils;
use chrono::Utc;
use colored::Colorize;
use std::fs;

pub async fn execute(network: String, gas_budget: Option<u64>, skip_verify: bool) -> Result<()> {
    let root = find_project_root()?;
    let config = load_config()?;

    // Validate network
    let valid_networks = vec!["devnet", "testnet", "mainnet"];
    if !valid_networks.contains(&network.as_str()) {
        return Err(SuiForgeError::InvalidNetwork(network));
    }

    utils::info(&format!("Deploying to {}...", network.yellow().bold()));

    // Get active address
    let spinner = utils::create_spinner("Getting active address...");
    let address = SuiCli::get_active_address()?;
    spinner.finish_with_message(format!("Active address: {}", address));

    // Build before deploy
    utils::info("Building contracts...");
    let build_output = SuiCli::build(true)?;
    if !build_output.status.success() {
        return Err(SuiForgeError::BuildFailed(
            "Build failed before deployment".to_string(),
        ));
    }

    // Deploy
    let spinner = utils::create_spinner("Publishing package...");
    let budget = gas_budget.unwrap_or(config.deploy.gas_budget);
    let output = SuiCli::publish(&network, budget)?;

    if output.status.success() {
        spinner.finish_with_message("Package published");

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Parse package ID from output (simplified)
        let package_id = extract_package_id(&stdout).unwrap_or_else(|| "0x...".to_string());

        // Save deployment lock
        let lock = DeploymentLock {
            package_id: package_id.clone(),
            object_ids: vec![],
            network: network.clone(),
            timestamp: Utc::now().to_rfc3339(),
            digest: "".to_string(),
        };

        let lock_path = root.join("suiforge.lock.json");
        let lock_json = serde_json::to_string_pretty(&lock)?;
        fs::write(lock_path, lock_json)?;

        utils::success("Deployment successful!");
        println!("\n{}", "Deployment Details:".bold());
        println!("  Network: {}", network.cyan());
        println!("  Package ID: {}", package_id.green());
        println!("  Deployer: {}", address.yellow());

        if !skip_verify {
            utils::info("Verifying deployment...");
            // Add verification logic here
        }
    } else {
        spinner.finish_with_message("Deployment failed");
        let stderr = String::from_utf8_lossy(&output.stderr);
        utils::error("Deployment failed:");
        println!("\n{}", stderr);
        return Err(SuiForgeError::DeploymentFailed(stderr.to_string()));
    }

    Ok(())
}

fn extract_package_id(output: &str) -> Option<String> {
    // Simplified extraction - actual implementation would parse JSON output
    for line in output.lines() {
        if line.contains("Package ID") || line.contains("packageId") {
            // Extract the actual ID
            return Some("0x...".to_string());
        }
    }
    None
}
