use crate::error::{Result, SuiForgeError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuiForgeConfig {
    pub network: NetworkConfig,
    pub build: BuildConfig,
    pub deploy: DeployConfig,
    pub codegen: CodegenConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub default: String,
    pub devnet: NetworkEndpoint,
    pub testnet: NetworkEndpoint,
    pub mainnet: Option<NetworkEndpoint>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkEndpoint {
    pub rpc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildConfig {
    #[serde(rename = "outputDir")]
    pub output_dir: String,
    #[serde(rename = "skipFetchLatestGitDeps")]
    pub skip_fetch_latest_git_deps: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployConfig {
    #[serde(rename = "gasObjectSelection")]
    pub gas_object_selection: String,
    #[serde(rename = "gasBudget")]
    pub gas_budget: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodegenConfig {
    pub typescript: Option<CodegenTarget>,
    pub rust: Option<CodegenTarget>,
    pub swift: Option<CodegenTarget>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodegenTarget {
    #[serde(rename = "outputDir")]
    pub output_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentLock {
    #[serde(rename = "packageId")]
    pub package_id: String,
    #[serde(rename = "objectIds")]
    pub object_ids: Vec<String>,
    pub network: String,
    pub timestamp: String,
    pub digest: String,
}

impl SuiForgeConfig {
    pub fn default() -> Self {
        Self {
            network: NetworkConfig {
                default: "devnet".to_string(),
                devnet: NetworkEndpoint {
                    rpc: "https://fullnode.devnet.sui.io:443".to_string(),
                },
                testnet: NetworkEndpoint {
                    rpc: "https://fullnode.testnet.sui.io:443".to_string(),
                },
                mainnet: Some(NetworkEndpoint {
                    rpc: "https://fullnode.mainnet.sui.io:443".to_string(),
                }),
            },
            build: BuildConfig {
                output_dir: "build".to_string(),
                skip_fetch_latest_git_deps: false,
            },
            deploy: DeployConfig {
                gas_object_selection: "auto".to_string(),
                gas_budget: 50000000,
            },
            codegen: CodegenConfig {
                typescript: Some(CodegenTarget {
                    output_dir: "./sdk/typescript".to_string(),
                }),
                rust: Some(CodegenTarget {
                    output_dir: "./sdk/rust".to_string(),
                }),
                swift: Some(CodegenTarget {
                    output_dir: "./sdk/swift".to_string(),
                }),
            },
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: SuiForgeConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

pub fn find_project_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;

    loop {
        let config_path = current.join("suiforge.config.json");
        if config_path.exists() {
            return Ok(current);
        }

        if !current.pop() {
            return Err(SuiForgeError::NotInProject);
        }
    }
}

pub fn load_config() -> Result<SuiForgeConfig> {
    let root = find_project_root()?;
    let config_path = root.join("suiforge.config.json");
    SuiForgeConfig::load(&config_path)
}
