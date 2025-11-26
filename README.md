# SuiForge

**A modern, developer-friendly framework for building on the Sui blockchain**

SuiForge simplifies the entire Sui smart contract development workflow—from scaffolding to deployment. Think Hardhat for Ethereum, Anchor for Solana, but built specifically for Sui Move.

## Why SuiForge?

The Sui ecosystem lacks modern, intuitive tooling. SuiForge solves this by:

- **Abstracting complexity**: No more wrestling with raw Sui CLI commands
- **Automating workflows**: One command to scaffold, build, test, and deploy
- **Providing reusable modules**: Battle-tested Move libraries for common patterns
- **Generating client SDKs**: Auto-generate TypeScript/Rust/Swift clients from your contracts
- **Offering great DX**: Human-friendly errors, sensible defaults, full customization

## Installation

```bash
cargo install suiforge
```

Or build from source:

```bash
git clone https://github.com/yourusername/suiforge
cd suiforge
cargo install --path .
```

## Quick Start

```bash
# Create a new project
suiforge init my-project --template nft

# Navigate to project
cd my-project

# Build the Move contracts
suiforge build

# Run tests
suiforge test

# Deploy to devnet
suiforge deploy devnet

# Generate TypeScript SDK
suiforge generate ts
```

## Commands

### `suiforge init <project-name>`
Scaffold a new Sui Move project with opinionated structure and templates.

**Options:**
- `--template <type>`: Choose from `basic`, `nft`, `token`, `marketplace`, `defi`, `game`
- `--no-git`: Skip git initialization

**Example:**
```bash
suiforge init my-nft-project --template nft
```

### `suiforge build`
Compile Move contracts with enhanced error messages.

**Options:**
- `--release`: Build with optimizations
- `--watch`: Rebuild on file changes

### `suiforge test`
Run Move unit tests and integration tests.

**Options:**
- `--filter <pattern>`: Run specific tests
- `--coverage`: Generate coverage report

### `suiforge deploy <network>`
Deploy contracts to specified network (devnet, testnet, mainnet).

**Options:**
- `--gas-budget <amount>`: Set custom gas budget
- `--skip-verify`: Skip post-deployment verification

**Example:**
```bash
suiforge deploy devnet
suiforge deploy mainnet --gas-budget 100000000
```

### `suiforge generate <target>`
Generate client SDKs from deployed contracts.

**Targets:** `ts`, `rust`, `swift`, `python`

**Example:**
```bash
suiforge generate ts --output ./sdk
```

### `suiforge node start|stop|status`
Manage local Sui node for development.

```bash
suiforge node start    # Start local node
suiforge node stop     # Stop local node
suiforge node status   # Check node status
```

### `suiforge install <plugin>`
Install community plugins (Phase 2).

```bash
suiforge install @suiforge/nft-tools
```

## Project Structure

```
my-project/
├── Move.toml                 # Move package manifest
├── suiforge.config.json      # SuiForge configuration
├── suiforge.lock.json        # Deployment metadata (auto-generated)
├── sources/
│   └── main.move             # Main contract code
├── tests/
│   └── main_tests.move       # Move unit tests
├── scripts/
│   └── deploy.ts             # Deployment scripts
└── build/                    # Build artifacts (auto-generated)
```

## Templates

SuiForge provides production-ready templates:

- **basic**: Minimal Move project
- **nft**: NFT collection with minting and transfers
- **token**: Fungible token with supply management
- **marketplace**: NFT marketplace with listings and sales
- **defi**: DeFi primitives (vaults, staking, swaps)
- **game**: On-chain game mechanics

## SuiForge Modules Library

Import battle-tested Move modules into your projects:

```toml
[dependencies]
SuiForge = { git = "https://github.com/yourusername/suiforge-modules", subdir = "modules" }
```

**Available modules:**
- `AccessControl`: Role-based permissions
- `Pausable`: Emergency stop mechanism
- `TokenUtils`: Fungible token helpers
- `NFTUtils`: Non-fungible token helpers
- `Vault`: Secure asset storage
- `Escrow`: Two-party escrow system
- `PaymentSplitter`: Revenue distribution
- `Ownable`: Ownership management

**Example usage:**
```move
use suiforge::access_control::{Self, Role};
use suiforge::pausable::{Self, Pausable};
```

## Configuration

`suiforge.config.json`:

```json
{
  "network": {
    "default": "devnet",
    "devnet": {
      "rpc": "https://fullnode.devnet.sui.io:443"
    },
    "testnet": {
      "rpc": "https://fullnode.testnet.sui.io:443"
    }
  },
  "build": {
    "outputDir": "build",
    "skipFetchLatestGitDeps": false
  },
  "deploy": {
    "gasObjectSelection": "auto",
    "gasBudget": 50000000
  },
  "codegen": {
    "typescript": {
      "outputDir": "./sdk/typescript"
    }
  }
}
```

## Best Practices

1. **Use templates**: Start with a template that matches your use case
2. **Leverage modules**: Don't reinvent the wheel—use SuiForge modules
3. **Test thoroughly**: Write comprehensive Move tests before deploying
4. **Version control**: Commit `suiforge.lock.json` to track deployments
5. **Generate SDKs**: Auto-generate type-safe clients for your frontend
6. **Local testing**: Use `suiforge node start` for fast iteration

## Contributing

SuiForge is open-source and welcomes contributions!

```bash
# Clone the repo
git clone https://github.com/yourusername/suiforge
cd suiforge

# Run tests
cargo test

# Build
cargo build --release
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Roadmap

- [x] Core CLI commands
- [x] Project scaffolding
- [x] Template system
- [x] Build & test automation
- [x] Deployment automation
- [x] SDK generation (TypeScript)
- [ ] Plugin ecosystem
- [ ] SDK generation (Rust, Swift, Python)
- [ ] Interactive mode
- [ ] Contract verification
- [ ] Gas optimization analyzer
- [ ] Security audit tools

## License

MIT License - see [LICENSE](LICENSE) for details.

## Support

- Documentation: https://suiforge.dev
- Discord: https://discord.gg/suiforge
- Twitter: @suiforge
- Issues: https://github.com/yourusername/suiforge/issues

---

Built with ❤️ for the Sui community
