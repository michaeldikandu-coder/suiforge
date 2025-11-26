# SuiForge Basic Usage Examples

This guide walks through common SuiForge workflows with practical examples.

## Example 1: Creating a Simple NFT Project

```bash
# Create a new NFT project
suiforge init my-nft-collection --template nft

# Navigate to project
cd my-nft-collection

# Review the generated code
cat sources/main.move

# Build the project
suiforge build

# Run tests
suiforge test

# Deploy to devnet
suiforge deploy devnet

# Generate TypeScript SDK
suiforge generate ts
```

### Generated Project Structure

```
my-nft-collection/
├── Move.toml
├── suiforge.config.json
├── sources/
│   └── main.move          # NFT implementation
├── tests/
│   └── main_tests.move    # Unit tests
├── scripts/
└── README.md
```

### Using the Generated NFT Module

```move
module my_nft_collection::nft {
    // Create a collection
    public fun create_collection(name: vector<u8>, ctx: &mut TxContext);
    
    // Mint an NFT
    public fun mint(
        collection: &mut Collection,
        name: vector<u8>,
        description: vector<u8>,
        url: vector<u8>,
        ctx: &mut TxContext
    );
    
    // Transfer NFT
    public fun transfer_nft(nft: NFT, recipient: address);
}
```

## Example 2: Creating a Token Project

```bash
# Create a fungible token project
suiforge init my-token --template token

cd my-token

# Build and test
suiforge build
suiforge test

# Deploy to testnet
suiforge deploy testnet --gas-budget 100000000
```

### Token Module Usage

```move
module my_token::token {
    // Mint new tokens
    public fun mint(
        treasury: &mut TreasuryCap<TOKEN>,
        amount: u64,
        recipient: address,
        ctx: &mut TxContext
    );
    
    // Burn tokens
    public fun burn(treasury: &mut TreasuryCap<TOKEN>, coin: Coin<TOKEN>);
}
```

## Example 3: Using SuiForge Modules

```bash
# Create a basic project
suiforge init my-vault --template basic

cd my-vault
```

### Add SuiForge modules to Move.toml

```toml
[dependencies]
Sui = { git = "https://github.com/MystenLabs/sui.git", subdir = "crates/sui-framework/packages/sui-framework", rev = "framework/mainnet" }
SuiForge = { git = "https://github.com/yourusername/suiforge", subdir = "modules" }
```

### Use modules in your code

```move
module my_vault::secure_vault {
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use suiforge::ownable::{Self, Ownable};
    use suiforge::pausable::{Self, Pausable};
    use suiforge::vault::{Self, Vault};

    public struct SecureVault has key {
        id: UID,
        ownable: Ownable,
        pausable: Pausable,
    }

    public fun create(ctx: &mut TxContext) {
        let vault = SecureVault {
            id: object::new(ctx),
            ownable: ownable::new(ctx),
            pausable: pausable::new(),
        };
        transfer::share_object(vault);
    }

    public fun pause(vault: &mut SecureVault, ctx: &TxContext) {
        ownable::require_owner(&vault.ownable, ctx);
        pausable::pause(&mut vault.pausable);
    }

    public fun unpause(vault: &mut SecureVault, ctx: &TxContext) {
        ownable::require_owner(&vault.ownable, ctx);
        pausable::unpause(&mut vault.pausable);
    }
}
```

## Example 4: Building a Marketplace

```bash
# Create marketplace project
suiforge init nft-marketplace --template marketplace

cd nft-marketplace

# Build
suiforge build

# Test
suiforge test

# Deploy to devnet
suiforge deploy devnet

# Generate TypeScript SDK for frontend
suiforge generate ts --output ./frontend/src/sdk
```

### Frontend Integration

```typescript
import { SuiClient } from '@mysten/sui.js/client';
import { SuiForgeClient } from './sdk';

const client = new SuiClient({ url: 'https://fullnode.devnet.sui.io:443' });
const packageId = '0x...'; // From suiforge.lock.json

const suiforge = new SuiForgeClient(client, packageId);

// Create transaction
const tx = suiforge.createTransaction();

// Add marketplace operations
// tx.moveCall({ ... });

// Execute transaction
const result = await client.signAndExecuteTransactionBlock({
  transactionBlock: tx,
  signer: keypair,
});
```

## Example 5: Local Development Workflow

```bash
# Start local Sui node
suiforge node start

# In another terminal, create and test project
suiforge init test-project
cd test-project

# Build and test locally
suiforge build
suiforge test

# Deploy to local node
suiforge deploy devnet

# Check node status
suiforge node status

# Stop node when done
suiforge node stop
```

## Example 6: Multi-Module Project

```bash
# Create basic project
suiforge init defi-protocol --template defi

cd defi-protocol
```

### Add multiple modules

```
sources/
├── vault.move
├── staking.move
├── rewards.move
└── governance.move
```

### Build and test all modules

```bash
# Build all modules
suiforge build

# Test specific module
suiforge test --filter vault

# Test all modules
suiforge test
```

## Example 7: Deployment to Multiple Networks

```bash
# Deploy to devnet
suiforge deploy devnet

# Test on devnet
# ... testing ...

# Deploy to testnet
suiforge deploy testnet

# Deploy to mainnet (with higher gas budget)
suiforge deploy mainnet --gas-budget 200000000
```

### Check deployment info

```bash
# View suiforge.lock.json
cat suiforge.lock.json
```

```json
{
  "packageId": "0x123...",
  "objectIds": ["0xabc...", "0xdef..."],
  "network": "devnet",
  "timestamp": "2025-01-01T12:00:00Z",
  "digest": "..."
}
```

## Example 8: Continuous Integration

### GitHub Actions Workflow

```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Sui
        run: cargo install --git https://github.com/MystenLabs/sui.git sui
      
      - name: Install SuiForge
        run: cargo install suiforge
      
      - name: Build
        run: suiforge build
      
      - name: Test
        run: suiforge test
```

## Example 9: Custom Configuration

### suiforge.config.json

```json
{
  "network": {
    "default": "devnet",
    "devnet": {
      "rpc": "https://fullnode.devnet.sui.io:443"
    },
    "custom": {
      "rpc": "http://localhost:9000"
    }
  },
  "build": {
    "outputDir": "build",
    "skipFetchLatestGitDeps": true
  },
  "deploy": {
    "gasObjectSelection": "auto",
    "gasBudget": 100000000
  },
  "codegen": {
    "typescript": {
      "outputDir": "./sdk/typescript"
    }
  }
}
```

## Best Practices

1. **Always test before deploying**
   ```bash
   suiforge test && suiforge deploy devnet
   ```

2. **Use version control**
   ```bash
   git add .
   git commit -m "Add new feature"
   ```

3. **Generate SDKs after deployment**
   ```bash
   suiforge deploy devnet && suiforge generate ts
   ```

4. **Keep dependencies updated**
   ```bash
   # Update Move.toml dependencies regularly
   ```

5. **Use templates as starting points**
   ```bash
   # Start with a template, then customize
   suiforge init my-project --template nft
   ```

## Troubleshooting

### Build Errors

```bash
# Clean build
rm -rf build/
suiforge build
```

### Deployment Issues

```bash
# Check active address
sui client active-address

# Check gas objects
sui client gas

# Increase gas budget
suiforge deploy devnet --gas-budget 100000000
```

### Test Failures

```bash
# Run specific test
suiforge test --filter test_name

# View detailed output
suiforge test 2>&1 | less
```

## Next Steps

- Read the [full documentation](https://suiforge.dev)
- Explore [advanced examples](./advanced-usage.md)
- Join the [Discord community](https://discord.gg/suiforge)
- Contribute to [SuiForge](https://github.com/yourusername/suiforge)
