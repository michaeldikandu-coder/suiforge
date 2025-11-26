# Changelog

All notable changes to SuiForge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Plugin ecosystem with registry
- Rust SDK generation
- Swift SDK generation
- Python SDK generation
- Watch mode for builds
- Coverage reporting
- Contract verification
- Gas optimization analyzer
- Security audit tools
- Interactive mode

## [0.1.0] - 2025-01-XX

### Added

#### Core Framework
- Complete CLI framework built with Rust
- Command-line argument parsing with Clap
- Async runtime with Tokio
- Comprehensive error handling with custom error types
- JSON-based configuration management
- Sui CLI wrapper for type-safe interactions

#### Commands
- `suiforge init` - Initialize new projects with template support
- `suiforge build` - Compile Move contracts with enhanced error messages
- `suiforge test` - Run Move tests with filtering options
- `suiforge deploy` - Deploy to devnet/testnet/mainnet with tracking
- `suiforge generate` - Generate client SDKs (TypeScript)
- `suiforge node` - Manage local Sui node (start/stop/status)
- `suiforge install` - Plugin installation (Phase 2 placeholder)

#### Project Templates
- **Basic**: Minimal Move project with simple objects
- **NFT**: Complete NFT collection with minting and transfers
- **Token**: Fungible token with treasury management
- **Marketplace**: NFT marketplace with listings and sales
- **DeFi**: Vault system for deposits and withdrawals
- **Game**: On-chain game with characters and progression

#### Move Modules Library
- **access_control**: Role-based access control with admin management
- **pausable**: Emergency stop mechanism for contracts
- **ownable**: Simple ownership management pattern
- **vault**: Secure token storage with deposit/withdraw
- **escrow**: Two-party escrow with release/refund functionality
- **payment_splitter**: Revenue distribution among multiple payees

#### Code Generation
- TypeScript SDK generator
- Automatic package.json generation
- Type-safe client wrapper classes
- Transaction builder utilities
- Type definitions for Move structs

#### Configuration
- `suiforge.config.json` for project settings
- `suiforge.lock.json` for deployment tracking
- Network configurations (devnet, testnet, mainnet)
- Build and deployment options
- Code generation settings

#### Developer Experience
- Colored terminal output for better readability
- Progress indicators for long-running operations
- Human-friendly error messages
- Helpful command documentation
- Git integration for new projects
- Automatic .gitignore generation

#### Documentation
- Comprehensive README with installation and usage
- Quick start guide for new users
- Architecture documentation with design decisions
- Contributing guidelines for developers
- Build guide with detailed instructions
- Project overview with vision and roadmap
- Usage examples with practical workflows
- Implementation summary

#### Build & Distribution
- Cargo.toml with all dependencies
- Makefile for build automation
- Installation scripts for Linux/macOS (install.sh)
- Installation scripts for Windows (install.ps1)
- .gitignore with sensible defaults
- MIT License

### Technical Details

#### Dependencies
- clap 4.5 - CLI argument parsing
- tokio 1.35 - Async runtime
- serde 1.0 - Serialization
- serde_json 1.0 - JSON handling
- anyhow 1.0 - Error handling
- thiserror 1.0 - Error derivation
- colored 2.1 - Terminal colors
- indicatif 0.17 - Progress bars
- dialoguer 0.11 - User prompts
- reqwest 0.11 - HTTP client
- toml 0.8 - TOML parsing
- handlebars 5.1 - Template engine
- walkdir 2.4 - Directory traversal
- chrono 0.4 - Date/time handling
- home 0.5 - Home directory
- which 6.0 - Executable finding
- async-trait 0.1 - Async traits
- tracing 0.1 - Logging
- tracing-subscriber 0.3 - Log subscriber

#### Code Statistics
- ~1,500 lines of Rust code
- ~600 lines of Move code
- ~3,000 lines of documentation
- 7 CLI commands
- 6 project templates
- 8 Move modules
- 1 SDK generator (TypeScript)

### Architecture

#### Design Patterns
- Command Pattern for CLI commands
- Wrapper Pattern for Sui CLI
- Template Method for project scaffolding
- Strategy Pattern for code generation
- Builder Pattern for configuration

#### Key Features
- Type-safe Rust implementation
- Modular architecture for extensibility
- Comprehensive error handling
- Async/await for performance
- Zero-copy where possible
- Minimal dependencies

### Breaking Changes
- None (initial release)

### Deprecated
- None (initial release)

### Removed
- None (initial release)

### Fixed
- None (initial release)

### Security
- Input validation for all user inputs
- Safe defaults for all operations
- No unsafe Rust code
- Dependency security auditing

## Version History

### [0.1.0] - Initial Release
First public release of SuiForge with core functionality:
- Complete CLI framework
- Project scaffolding with templates
- Build and test automation
- Deployment automation
- TypeScript SDK generation
- Move modules library
- Comprehensive documentation

## Upgrade Guide

### From Nothing to 0.1.0

This is the initial release. To get started:

```bash
# Install SuiForge
cargo install suiforge

# Create a new project
suiforge init my-project --template nft

# Build and deploy
cd my-project
suiforge build
suiforge test
suiforge deploy devnet
```

## Future Roadmap

### Version 0.2.0 (Q2 2025)
- Plugin ecosystem
- Rust SDK generation
- Swift SDK generation
- Watch mode for builds
- Coverage reporting

### Version 0.3.0 (Q3 2025)
- Contract verification
- Gas optimization analyzer
- Security audit tools
- Interactive mode
- Advanced templates

### Version 1.0.0 (Q4 2025)
- Stable API
- Enterprise features
- Multi-chain support
- Advanced testing tools
- Performance optimizations

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to SuiForge.

## Support

- **Documentation**: https://suiforge.dev
- **GitHub Issues**: https://github.com/yourusername/suiforge/issues
- **Discord**: https://discord.gg/suiforge
- **Twitter**: @suiforge

---

[Unreleased]: https://github.com/yourusername/suiforge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/suiforge/releases/tag/v0.1.0
