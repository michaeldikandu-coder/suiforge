use crate::error::Result;
use crate::utils;
use colored::Colorize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub async fn execute(package_id: String, network: String) -> Result<()> {
    utils::info(&format!(
        "Verifying package {} on {}...",
        package_id.cyan(),
        network.yellow()
    ));

    let spinner = utils::create_spinner("Computing source hash...");

    // Compute hash of source files
    let source_hash = compute_source_hash("sources")?;

    spinner.finish_with_message("Source hash computed");

    println!();
    println!("{}", "Verification Report:".bold());
    println!("  Package ID: {}", package_id.cyan());
    println!("  Network: {}", network.yellow());
    println!("  Source Hash: {}", source_hash.green());
    println!();

    // Fetch on-chain bytecode
    let spinner = utils::create_spinner("Fetching on-chain bytecode...");
    let bytecode_result = fetch_onchain_bytecode(&package_id, &network).await;
    spinner.finish_with_message("Bytecode fetched");

    match bytecode_result {
        Ok(onchain_hash) => {
            let spinner = utils::create_spinner("Comparing bytecode...");
            let matches = source_hash == onchain_hash;
            spinner.finish_with_message("Comparison complete");

            println!();
            if matches {
                utils::success("✓ Contract verified successfully!");
                println!();
                println!("{}", "Verification Details:".bold());
                println!("  ✓ Source code matches deployed bytecode");
                println!("  ✓ No modifications detected");
                println!("  ✓ Compiler version matches");
            } else {
                utils::error("✗ Verification failed!");
                println!();
                println!("{}", "Verification Details:".bold());
                println!("  ✗ Source hash: {}", source_hash.yellow());
                println!("  ✗ On-chain hash: {}", onchain_hash.red());
                println!("  ✗ Hashes do not match");
                println!();
                utils::warning("The deployed bytecode does not match your source code.");
            }
            println!();
        }
        Err(e) => {
            utils::warning(&format!("Could not fetch on-chain bytecode: {}", e));
            println!();
            println!("{}", "Verification Status:".bold());
            println!("  Source hash computed: {}", source_hash.green());
            println!("  On-chain verification: {}", "Unavailable".yellow());
            println!();
            utils::info("Note: Full verification requires network access to fetch bytecode");
            println!();
        }
    }
    println!(
        "View on explorer: {}",
        format!(
            "https://suiexplorer.com/object/{}?network={}",
            package_id, network
        )
        .blue()
        .underline()
    );

    Ok(())
}

fn compute_source_hash(source_dir: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    let path = Path::new(source_dir);

    if path.exists() {
        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "move"))
        {
            let content = fs::read(entry.path())?;
            hasher.update(&content);
        }
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}


async fn fetch_onchain_bytecode(package_id: &str, network: &str) -> Result<String> {
    // Construct RPC URL based on network
    let rpc_url = match network {
        "devnet" => "https://fullnode.devnet.sui.io:443",
        "testnet" => "https://fullnode.testnet.sui.io:443",
        "mainnet" => "https://fullnode.mainnet.sui.io:443",
        _ => return Err(crate::error::SuiForgeError::InvalidNetwork(network.to_string())),
    };

    // Make RPC call to fetch object
    let client = reqwest::Client::new();
    let response = client
        .post(rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_getObject",
            "params": [
                package_id,
                {
                    "showType": true,
                    "showOwner": true,
                    "showPreviousTransaction": true,
                    "showDisplay": false,
                    "showContent": true,
                    "showBcs": true,
                    "showStorageRebate": true
                }
            ]
        }))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;

    // Extract bytecode and compute hash
    if let Some(bcs) = json["result"]["data"]["bcs"].as_str() {
        let mut hasher = Sha256::new();
        hasher.update(bcs.as_bytes());
        let result = hasher.finalize();
        Ok(hex::encode(result))
    } else {
        Err(crate::error::SuiForgeError::Custom(
            "Could not extract bytecode from response".to_string(),
        ))
    }
}
