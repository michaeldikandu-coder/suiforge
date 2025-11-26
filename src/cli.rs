use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "suiforge")]
#[command(author, version, about = "A modern developer framework for Sui blockchain", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new SuiForge project
    Init {
        /// Project name
        name: String,
        
        /// Template to use (basic, nft, token, marketplace, defi, game)
        #[arg(short, long, default_value = "basic")]
        template: String,
        
        /// Skip git initialization
        #[arg(long)]
        no_git: bool,
    },
    
    /// Build Move contracts
    Build {
        /// Build with optimizations
        #[arg(short, long)]
        release: bool,
        
        /// Watch for changes and rebuild
        #[arg(short, long)]
        watch: bool,
    },
    
    /// Run Move tests
    Test {
        /// Filter tests by pattern
        #[arg(short, long)]
        filter: Option<String>,
        
        /// Generate coverage report
        #[arg(short, long)]
        coverage: bool,
    },
    
    /// Deploy contracts to network
    Deploy {
        /// Target network (devnet, testnet, mainnet)
        network: String,
        
        /// Gas budget for deployment
        #[arg(short, long)]
        gas_budget: Option<u64>,
        
        /// Skip post-deployment verification
        #[arg(long)]
        skip_verify: bool,
    },
    
    /// Generate client SDKs
    Generate {
        /// Target language (ts, rust, swift, python)
        target: String,
        
        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Manage local Sui node
    Node {
        /// Action (start, stop, status)
        action: String,
    },
    
    /// Install SuiForge plugin
    Install {
        /// Plugin name
        plugin: String,
    },
}
