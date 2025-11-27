# SuiForge v0.2.0 - Advanced Features Guide

Complete guide to the advanced features added in SuiForge v0.2.0 that go beyond the native Sui CLI.

## 🆕 What's New in v0.2.0

SuiForge v0.2.0 introduces 9 powerful features that significantly enhance the Sui development experience:

1. **Multi-Network Profile System** - Manage multiple network configurations
2. **Contract Verification Engine** - Verify deployed contracts match source code
3. **Gas Profiler** - Analyze and optimize gas usage
4. **Move Security Scanner** - Detect security vulnerabilities
5. **Watch Mode** - Auto-rebuild on file changes
6. **Debugging Dashboard** - Visual debugging interface
7. **Enhanced Plugin System** - Extensible architecture
8. **State Inspector** - Deep dive into contract state
9. **Test Coverage Reports** - Comprehensive coverage analysis

---

## 1. 🌐 Multi-Network Profile System

Manage multiple network configurations with ease.

### Commands

```bash
# List all profiles
suiforge profile list

# Add a new profile
suiforge profile add --name custom --rpc https://my-node.example.com:443

# Switch active profile
suiforge profile switch --name testnet

# Show profile details
suiforge profile show --name devnet

# Remove a profile
suiforge profile remove --name custom
```

### Example Output

```
ℹ Network Profiles:

● devnet https://fullnode.devnet.sui.io:443
    Faucet: https://faucet.devnet.sui.io/gas
○ testnet https://fullnode.testnet.sui.io:443
    Faucet: https://faucet.testnet.sui.io/gas
○ mainnet https://fullnode.mainnet.sui.io:443

ℹ Active profile: devnet
```

### Configuration

Profiles are stored in `~/.suiforge/profiles.json`:

```json
{
  "active": "devnet",
  "profiles": {
    "devnet": {
      "name": "devnet",
      "rpc": "https://fullnode.devnet.sui.io:443",
      "faucet": "https://faucet.devnet.sui.io/gas",
      "explorer": "https://suiexplorer.com/?network=devnet"
    }
  }
}
```

---

## 2. ✅ Contract Verification Engine

Verify that deployed contracts match your source code.

### Command

```bash
# Verify a deployed contract
suiforge verify 0x123abc... --network devnet
```

### Example Output

```
ℹ Verifying package 0x123abc... on devnet...
⠋ Computing source hash...
✓ Source hash computed

Verification Report:
  Package ID: 0x123abc...
  Network: devnet
  Source Hash: a1b2c3d4e5f6...

⠋ Fetching on-chain bytecode...
✓ Bytecode fetched
⠋ Comparing bytecode...
✓ Comparison complete

✓ Contract verified successfully!

Verification Details:
  ✓ Source code matches deployed bytecode
  ✓ No modifications detected
  ✓ Compiler version matches

View on explorer: https://suiexplorer.com/object/0x123abc...?network=devnet
```

### How It Works

1. Computes SHA-256 hash of all source files
2. Fetches deployed bytecode from the network
3. Compares source hash with on-chain bytecode
4. Reports any discrepancies

---

## 3. ⛽ Gas Profiler

Analyze and optimize gas usage in your contracts.

### Commands

```bash
# Profile gas usage
suiforge gas profile

# Profile specific function
suiforge gas profile --function create_nft

# Analyze gas patterns
suiforge gas analyze

# Get optimization suggestions
suiforge gas optimize
```

### Example Output - Profile

```
ℹ Profiling gas usage...
⠋ Analyzing Move bytecode...
✓ Analysis complete

Gas Usage Profile:

Function                  Total Gas      Storage    Computation
──────────────────────────────────────────────────────────────────
create_nft                  1250 gas       800 gas        450 gas
transfer_nft                 650 gas       200 gas        450 gas
mint_token                   980 gas       600 gas        380 gas

ℹ 💡 Tip: Use 'suiforge gas optimize' for optimization suggestions
```

### Example Output - Optimize

```
ℹ Generating optimization suggestions...
⠋ Analyzing code patterns...
✓ Analysis complete

🚀 Gas Optimization Suggestions:

1. Reduce Storage Allocations
   Location: sources/nft.move:45
   Current: Creating new object for each NFT
   Suggestion: Use shared objects or batch operations
   💰 Potential savings: ~200 gas per call

2. Optimize Vector Operations
   Location: sources/marketplace.move:78
   Current: Multiple vector iterations
   Suggestion: Combine iterations or use table lookups
   💰 Potential savings: ~150 gas per call

3. Cache Computed Values
   Location: sources/rewards.move:112
   Current: Recalculating values in loop
   Suggestion: Store intermediate results
   💰 Potential savings: ~100 gas per call

✓ Total potential savings: ~450 gas per transaction
```

---

## 4. 🔒 Move Security Scanner

Detect security vulnerabilities in your Move code.

### Commands

```bash
# Run security scan
suiforge scan

# Scan with specific level
suiforge scan --level strict

# Output as JSON
suiforge scan --format json
```

### Example Output

```
ℹ Running security scan (level: standard)...
⠋ Analyzing Move code...
✓ Found 3 potential issues

🔒 Security Scan Report
══════════════════════════════════════════════════════════════════

Summary:
  🔴 1 critical issues
  🟠 1 high severity issues
  🟡 1 medium severity issues

Issue #1
  Severity: CRITICAL
  Title: Reentrancy Risk
  Location: sources/vault.move:89
  Description: External call before state update
  Recommendation: Update state before making external calls (checks-effects-interactions)

Issue #2
  Severity: HIGH
  Title: Unchecked Transfer
  Location: sources/nft.move:67
  Description: Transfer operation without ownership verification
  Recommendation: Add ownership check before transfer: assert!(owner == sender)

Issue #3
  Severity: MEDIUM
  Title: Missing Access Control
  Location: sources/marketplace.move:45
  Description: Public function without role-based access control
  Recommendation: Implement admin-only modifier or capability pattern

⚠️  Critical or high severity issues found! Please review before deployment.
```

### Scan Levels

- **basic**: Critical and high severity issues only
- **standard**: Critical, high, and medium severity issues
- **strict**: All issues including low severity and informational

---

## 5. 👀 Watch Mode

Automatically rebuild (and optionally test) when files change.

### Commands

```bash
# Watch and rebuild
suiforge watch

# Watch, rebuild, and test
suiforge watch --test

# Watch with deploy flag (requires confirmation)
suiforge watch --test --deploy
```

### Example Output

```
ℹ Starting watch mode...
  Watching: sources/
  Auto-test: enabled

Press Ctrl+C to stop

ℹ Change detected at 14:32:15

ℹ Building Move contracts...
⠋ Compiling Move code...
✓ Move contracts built successfully!

ℹ Running Move tests...
⠋ Executing tests...
✓ All tests passed!

✓ Ready for changes
```

### Features

- Debounced file watching (500ms)
- Automatic rebuild on source changes
- Optional test execution
- Clear, timestamped output
- Graceful error handling

---

## 6. 🎛️ Debugging Dashboard

Launch an interactive web-based debugging dashboard.

### Command

```bash
# Start dashboard on default port (3000)
suiforge dashboard

# Start on custom port
suiforge dashboard --port 8080
```

### Example Output

```
ℹ Starting debugging dashboard on port 3000...

🎛️  SuiForge Dashboard
══════════════════════════════════════════════════

Dashboard Features:
  • 📊 Real-time contract state monitoring
  • 📜 Transaction history and analysis
  • ⛽ Gas usage visualization
  • 📡 Event log streaming
  • 🔧 Interactive contract calls
  • 🌐 Network status monitoring

Access URLs:
  Local:    http://localhost:3000
  Network:  http://0.0.0.0:3000

⠋ Initializing dashboard...
✓ Dashboard ready

✓ Dashboard running at http://localhost:3000

Quick Actions:
  • View contracts: http://localhost:3000/contracts
  • Monitor gas: http://localhost:3000/gas
  • Event logs: http://localhost:3000/events

Press Ctrl+C to stop the dashboard
```

### Dashboard Features

- Real-time contract state monitoring
- Transaction history and analysis
- Gas usage visualization
- Event log streaming
- Interactive contract calls
- Network status monitoring

---

## 7. 🔍 State Inspector

Deep dive into contract state and object data.

### Commands

```bash
# Inspect object (tree format)
suiforge inspect 0x123abc...

# Inspect with specific network
suiforge inspect 0x123abc... --network testnet

# Output as JSON
suiforge inspect 0x123abc... --format json

# Simple text format
suiforge inspect 0x123abc... --format text
```

### Example Output - Tree Format

```
ℹ Inspecting object 0x123abc... on devnet...
⠋ Fetching object data...
✓ Object data retrieved

📦 Object Inspector
════════════════════════════════════════════════════════════

🔍 Object Details
├─ ID: 0x123abc...
├─ Network: devnet
├─ Type: 0x2::nft::NFT
├─ Owner: 0x456def...
└─ Version: 42

📊 Object Data
├─ name: "Cool NFT #1"
├─ description: "A unique digital collectible"
├─ image_url: "https://example.com/nft/1.png"
├─ attributes
│  ├─ rarity: "legendary"
│  ├─ power: 95
│  └─ level: 10
└─ created_at: 1640000000

⛽ Gas & Storage
├─ Storage Cost: 1,200 MIST
├─ Storage Rebate: 800 MIST
└─ Net Cost: 400 MIST

🔗 References
├─ Parent: 0xabc...def
└─ Children: None

ℹ View on explorer: https://suiexplorer.com/object/0x123abc...?network=devnet
```

---

## 8. 📊 Test Coverage Reports

Generate comprehensive test coverage reports.

### Commands

```bash
# Generate HTML report (default)
suiforge coverage

# Generate text report
suiforge coverage --format text

# Generate JSON report
suiforge coverage --format json

# Custom output directory
suiforge coverage --output ./my-coverage
```

### Example Output - Text Format

```
ℹ Generating test coverage report...
⠋ Running tests with coverage...
✓ Tests completed
⠋ Analyzing coverage data...
✓ Analysis complete

📊 Test Coverage Report
══════════════════════════════════════════════════════════════════

Overall Coverage:
  Lines: 1,234 / 1,448 (85.2%)
  Functions: 37 / 40 (92.5%)
  Branches: 156 / 199 (78.3%)

Coverage by Module:

Module                           Lines  Functions   Branches
──────────────────────────────────────────────────────────────────
sources/nft.move                 95.2%      100%      88.9%
sources/marketplace.move         87.5%       90%      75.0%
sources/token.move               92.1%       95%      82.5%
sources/vault.move               78.3%       85%      70.2%
sources/rewards.move             81.7%     88.9%      73.8%

Uncovered Lines:
  sources/vault.move:45-48 → Withdrawal logic
  sources/vault.move:67 → Error handling
  sources/rewards.move:89-92 → Edge case

✓ Coverage report generated

ℹ 💡 Tip: Aim for >80% coverage for production code
```

### HTML Report

The HTML report includes:
- Interactive coverage visualization
- Line-by-line coverage highlighting
- Sortable tables
- Coverage trends over time
- Exportable data

---

## 🔧 Integration with Existing Commands

All new features integrate seamlessly with existing SuiForge commands:

### Build with Watch

```bash
# Old way
suiforge build

# New way - auto-rebuild
suiforge watch
```

### Deploy with Verification

```bash
# Deploy
suiforge deploy devnet

# Then verify
suiforge verify 0x123abc... --network devnet
```

### Test with Coverage

```bash
# Old way
suiforge test

# New way - with coverage
suiforge coverage
```

---

## 📝 Configuration

### Global Configuration

New features use `~/.suiforge/` for global configuration:

```
~/.suiforge/
├── profiles.json       # Network profiles
├── plugins/            # Installed plugins
└── cache/              # Cached data
```

### Project Configuration

Enhanced `suiforge.config.json`:

```json
{
  "network": {
    "default": "devnet"
  },
  "security": {
    "scan_level": "standard",
    "auto_scan": true
  },
  "gas": {
    "profile_on_build": false,
    "optimization_level": "standard"
  },
  "coverage": {
    "threshold": 80,
    "exclude": ["tests/"]
  },
  "watch": {
    "auto_test": true,
    "debounce_ms": 500
  }
}
```

---

## 🚀 Best Practices

### 1. Use Profiles for Different Environments

```bash
suiforge profile add --name staging --rpc https://staging.example.com
suiforge profile switch --name staging
suiforge deploy staging
```

### 2. Run Security Scans Before Deployment

```bash
suiforge scan --level strict
suiforge deploy devnet
suiforge verify 0x123abc... --network devnet
```

### 3. Monitor Gas Usage

```bash
suiforge gas profile
suiforge gas optimize
# Apply optimizations
suiforge gas profile  # Verify improvements
```

### 4. Use Watch Mode During Development

```bash
suiforge watch --test
# Edit code, save, and see results automatically
```

### 5. Maintain High Test Coverage

```bash
suiforge coverage
# Aim for >80% coverage
# Add tests for uncovered lines
```

---

## 🔮 Coming Soon

Future enhancements planned:

- **AI-Powered Optimization**: ML-based gas optimization suggestions
- **Fuzzing Integration**: Automated fuzz testing
- **Formal Verification**: Mathematical proof of correctness
- **Performance Benchmarking**: Comparative performance analysis
- **Multi-Chain Support**: Deploy to multiple networks simultaneously
- **Contract Upgradeability**: Managed upgrade patterns
- **Dependency Analysis**: Visualize contract dependencies

---

## 📚 Additional Resources

- [Main README](README.md) - Getting started guide
- [Architecture](ARCHITECTURE.md) - Technical architecture
- [Contributing](CONTRIBUTING.md) - Contribution guidelines
- [Examples](examples/basic-usage.md) - Usage examples

---

**SuiForge v0.2.0** - Building the future of Sui development 🚀
