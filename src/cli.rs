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

    /// Manage network profiles
    Profile {
        /// Action (list, add, remove, switch, show)
        action: String,

        /// Profile name
        #[arg(short, long)]
        name: Option<String>,

        /// RPC URL for new profile
        #[arg(short, long)]
        rpc: Option<String>,
    },

    /// Verify deployed contract
    Verify {
        /// Package ID to verify
        package_id: String,

        /// Network to verify on
        #[arg(short, long, default_value = "devnet")]
        network: String,
    },

    /// Profile gas usage
    Gas {
        /// Action (profile, analyze, optimize)
        action: String,

        /// Function to profile
        #[arg(short, long)]
        function: Option<String>,
    },

    /// Security scan Move code
    Scan {
        /// Scan level (basic, standard, strict)
        #[arg(short, long, default_value = "standard")]
        level: String,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Watch mode for auto-rebuild
    Watch {
        /// Also run tests on change
        #[arg(short, long)]
        test: bool,

        /// Also deploy on change
        #[arg(short, long)]
        deploy: bool,
    },

    /// Launch debugging dashboard
    Dashboard {
        /// Port to run on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },

    /// Inspect contract state
    Inspect {
        /// Object ID to inspect
        object_id: String,

        /// Network
        #[arg(short, long, default_value = "devnet")]
        network: String,

        /// Output format (text, json, tree)
        #[arg(short, long, default_value = "tree")]
        format: String,
    },

    /// Generate test coverage report
    Coverage {
        /// Output format (html, text, json)
        #[arg(short, long, default_value = "html")]
        format: String,

        /// Output directory
        #[arg(short, long, default_value = "./coverage")]
        output: String,
    },
}
