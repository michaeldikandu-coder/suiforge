use crate::error::Result;
use crate::utils;
use colored::Colorize;

pub async fn execute(object_id: String, network: String, format: String) -> Result<()> {
    utils::info(&format!(
        "Inspecting object {} on {}...",
        object_id.cyan(),
        network.yellow()
    ));

    let spinner = utils::create_spinner("Fetching object data...");
    let object_data = fetch_object_data(&object_id, &network).await?;
    spinner.finish_with_message("Object data retrieved");

    match format.as_str() {
        "json" => print_json_format(&object_data).await,
        "tree" => print_tree_format(&object_data).await,
        _ => print_text_format(&object_data).await,
    }
}

async fn fetch_object_data(object_id: &str, network: &str) -> Result<serde_json::Value> {
    let rpc_url = match network {
        "devnet" => "https://fullnode.devnet.sui.io:443",
        "testnet" => "https://fullnode.testnet.sui.io:443",
        "mainnet" => "https://fullnode.mainnet.sui.io:443",
        _ => return Err(crate::error::SuiForgeError::InvalidNetwork(network.to_string())),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_getObject",
            "params": [
                object_id,
                {
                    "showType": true,
                    "showOwner": true,
                    "showPreviousTransaction": true,
                    "showDisplay": false,
                    "showContent": true,
                    "showBcs": false,
                    "showStorageRebate": true
                }
            ]
        }))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;

    if let Some(error) = json.get("error") {
        return Err(crate::error::SuiForgeError::Custom(format!(
            "RPC error: {}",
            error
        )));
    }

    Ok(json)
}

async fn print_tree_format(data: &serde_json::Value) -> Result<()> {
    println!();
    println!("{}", "📦 Object Inspector".bold());
    println!("{}", "═".repeat(60));
    println!();

    let result = &data["result"]["data"];
    let object_id = result["objectId"].as_str().unwrap_or("unknown");
    let object_type = result["type"].as_str().unwrap_or("unknown");
    let owner = result["owner"].as_object()
        .and_then(|o| o.get("AddressOwner"))
        .and_then(|a| a.as_str())
        .unwrap_or("unknown");
    let version = result["version"].as_str().unwrap_or("unknown");

    println!("🔍 {}", "Object Details".bold());
    println!("├─ ID: {}", object_id.cyan());
    println!("├─ Type: {}", object_type.green());
    println!("├─ Owner: {}", owner.blue());
    println!("└─ Version: {}", version.dimmed());
    println!();

    if let Some(content) = result["content"].as_object() {
        println!("📊 {}", "Object Data".bold());
        if let Some(fields) = content["fields"].as_object() {
            print_fields(fields, "");
        }
        println!();
    }

    if let Some(storage_rebate) = result["storageRebate"].as_str() {
        println!("⛽ {}", "Gas & Storage".bold());
        println!("├─ Storage Rebate: {}", storage_rebate.green());
        println!();
    }

    utils::info(&format!(
        "View on explorer: {}",
        format!("https://suiexplorer.com/object/{}", object_id)
            .blue()
            .underline()
    ));

    Ok(())
}

fn print_fields(fields: &serde_json::Map<String, serde_json::Value>, prefix: &str) {
    let keys: Vec<_> = fields.keys().collect();
    for (i, key) in keys.iter().enumerate() {
        let is_last = i == keys.len() - 1;
        let branch = if is_last { "└─" } else { "├─" };
        let value = &fields[*key];

        match value {
            serde_json::Value::Object(obj) => {
                println!("{}{} {}", prefix, branch, key.cyan());
                let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
                print_fields(obj, &new_prefix);
            }
            serde_json::Value::String(s) => {
                println!("{}{} {}: {}", prefix, branch, key.cyan(), s.green());
            }
            serde_json::Value::Number(n) => {
                println!("{}{} {}: {}", prefix, branch, key.cyan(), n.to_string().yellow());
            }
            serde_json::Value::Bool(b) => {
                println!("{}{} {}: {}", prefix, branch, key.cyan(), b.to_string().blue());
            }
            _ => {
                println!("{}{} {}: {}", prefix, branch, key.cyan(), value.to_string().dimmed());
            }
        }
    }
}

async fn print_text_format(data: &serde_json::Value) -> Result<()> {
    let result = &data["result"]["data"];
    
    println!();
    println!("{}", "Object Details:".bold());
    println!("  ID: {}", result["objectId"].as_str().unwrap_or("unknown").cyan());
    println!("  Type: {}", result["type"].as_str().unwrap_or("unknown").green());
    
    if let Some(owner) = result["owner"].as_object() {
        if let Some(address) = owner.get("AddressOwner").and_then(|a| a.as_str()) {
            println!("  Owner: {}", address.blue());
        }
    }
    
    println!("  Version: {}", result["version"].as_str().unwrap_or("unknown").dimmed());
    println!();

    if let Some(content) = result["content"].as_object() {
        println!("{}", "Data:".bold());
        if let Some(fields) = content["fields"].as_object() {
            for (key, value) in fields {
                println!("  {}: {}", key, value);
            }
        }
        println!();
    }

    Ok(())
}

async fn print_json_format(data: &serde_json::Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&data["result"])?);
    Ok(())
}
