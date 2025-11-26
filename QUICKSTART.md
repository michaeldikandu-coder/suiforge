# SuiForge Quick Start Guide

Get up and running with SuiForge in 5 minutes.

## Prerequisites

1. **Install Rust** (if not already installed)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Sui CLI**
   ```bash
   cargo install --git https://github.com/MystenLabs/sui.git sui
   ```

3. **Configure Sui**
   ```bash
   sui client
   # Follow prompts to set up wallet and network
   ```

## Install SuiForge

### Option 1: From Cargo (Recommended)

```bash
cargo install suiforge
```

### Option 2: From Source

```bash
git clone https://github.com/yourusername/suiforge
cd suiforge
cargo install --path .
```

### Verify Installation

```bash
suiforge --version
```

## Create Your First Project

### 1. Initialize a New Project

```bash
suiforge init my-first-project --template nft
cd my-first-project
```

This creates:
```
my-first-project/
├── Move.toml              # Move package configuration
├── suiforge.config.json   # SuiForge settings
├── sources/
│   └── main.move          # Your smart contract
├── tests/
│   └── main_tests.move    # Unit tests
└── README.md
```

### 2. Review the Generated Code

```bash
cat sources/main.move
```

You'll see a complete NFT implementation with:
- Collection creation
- NFT minting
- Transfer functionality
- Event emission

### 3. Build the Project

```bash
suiforge build
```

Expected output:
```
ℹ Building Move contracts...
⠋ Compiling Move code...
✓ Move contracts built successfully!
```

### 4. Run Tests

```bash
suiforge test
```

Expected output:
```
ℹ Running Move tests...
⠋ Executing tests...
✓ All tests passed!
```

### 5. Deploy to Devnet

```bash
suiforge deploy devnet
```

Expected output:
```
ℹ Deploying to devnet...
⠋ Getting active address...
✓ Active address: 0x123...
ℹ Building contracts...
⠋ Publishing package...
✓ Deployment successful!

Deployment Details:
  Network: devnet
  Package ID: 0xabc...
  Deployer: 0x123...
```

### 6. Generate TypeScript SDK

```bash
suiforge generate ts
```

This creates a TypeScript SDK in `./sdk/typescript/` that you can use in your frontend.

## Next Steps

### Explore Templates

Try different project templates:

```bash
# Token project
suiforge init my-token --template token

# Marketplace
suiforge init my-marketplace --template marketplace

# DeFi protocol
suiforge init my-defi --template defi

# Game
suiforge init my-game --template game
```

### Use SuiForge Modules

Add reusable modules to your project:

1. Update `Move.toml`:
   ```toml
   [dependencies]
   SuiForge = { git = "https://github.com/yourusername/suiforge", subdir = "modules" }
   ```

2. Import in your code:
   ```move
   use suiforge::access_control;
   use suiforge::pausable;
   use suiforge::vault;
   ```

### Local Development

Start a local Sui node:

```bash
suiforge node start
```

Deploy to local node:

```bash
suiforge deploy devnet
```

### Customize Configuration

Edit `suiforge.config.json`:

```json
{
  "network": {
    "default": "devnet"
  },
  "deploy": {
    "gasBudget": 100000000
  }
}
```

## Common Commands

```bash
# Create project
suiforge init <name> --template <type>

# Build contracts
suiforge build

# Run tests
suiforge test

# Deploy
suiforge deploy <network>

# Generate SDK
suiforge generate <target>

# Manage local node
suiforge node start|stop|status

# Get help
suiforge --help
suiforge <command> --help
```

## Example: Complete Workflow

```bash
# 1. Create project
suiforge init my-nft --template nft
cd my-nft

# 2. Customize the code
# Edit sources/main.move

# 3. Build and test
suiforge build
suiforge test

# 4. Deploy to devnet
suiforge deploy devnet

# 5. Generate SDK for frontend
suiforge generate ts --output ../frontend/src/sdk

# 6. Use in your app
cd ../frontend
npm install
# Import and use the generated SDK
```

## Troubleshooting

### "Sui CLI not found"

Install Sui CLI:
```bash
cargo install --git https://github.com/MystenLabs/sui.git sui
```

### "Not in a SuiForge project directory"

Make sure you're in a directory with `suiforge.config.json`:
```bash
cd my-project
suiforge build
```

### Build Errors

Clean and rebuild:
```bash
rm -rf build/
suiforge build
```

### Deployment Fails

Check your gas budget:
```bash
suiforge deploy devnet --gas-budget 100000000
```

Check your active address has funds:
```bash
sui client gas
```

Get devnet tokens:
```bash
curl --location --request POST 'https://faucet.devnet.sui.io/gas' \
  --header 'Content-Type: application/json' \
  --data-raw '{"FixedAmountRequest":{"recipient":"YOUR_ADDRESS"}}'
```

## Learning Resources

- **Documentation**: https://suiforge.dev
- **Examples**: See `examples/` directory
- **Sui Docs**: https://docs.sui.io
- **Move Book**: https://move-book.com
- **Discord**: https://discord.gg/suiforge

## What's Next?

1. **Read the full README**: Learn about all features
2. **Explore examples**: Check `examples/basic-usage.md`
3. **Study the architecture**: Read `ARCHITECTURE.md`
4. **Contribute**: See `CONTRIBUTING.md`
5. **Join the community**: Discord, Twitter, GitHub

## Getting Help

- **GitHub Issues**: Report bugs or request features
- **Discord**: Ask questions and get help
- **Documentation**: Comprehensive guides and API docs
- **Examples**: Real-world usage examples

---

Happy building with SuiForge! 🚀
