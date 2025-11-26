use thiserror::Error;

pub type Result<T> = std::result::Result<T, SuiForgeError>;

#[derive(Error, Debug)]
pub enum SuiForgeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),
    
    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    
    #[error("Template error: {0}")]
    Template(#[from] handlebars::RenderError),
    
    #[error("Project already exists at: {0}")]
    ProjectExists(String),
    
    #[error("Not in a SuiForge project directory")]
    NotInProject,
    
    #[error("Invalid template: {0}")]
    InvalidTemplate(String),
    
    #[error("Invalid network: {0}")]
    InvalidNetwork(String),
    
    #[error("Sui CLI not found. Please install Sui CLI first.")]
    #[allow(dead_code)]
    SuiCliNotFound,
    
    #[error("Build failed: {0}")]
    BuildFailed(String),
    
    #[error("Test failed: {0}")]
    TestFailed(String),
    
    #[error("Deployment failed: {0}")]
    DeploymentFailed(String),
    
    #[error("Code generation failed: {0}")]
    CodegenFailed(String),
    
    #[error("Configuration error: {0}")]
    #[allow(dead_code)]
    ConfigError(String),
    
    #[error("{0}")]
    Custom(String),
}
