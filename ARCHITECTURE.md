# SuiForge Architecture

This document describes the architecture and design decisions of SuiForge.

## Overview

SuiForge is a Rust-based CLI framework that wraps and enhances the Sui CLI to provide a better developer experience for building Move smart contracts on Sui.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        SuiForge CLI                          │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │   Init   │  │  Build   │  │   Test   │  │  Deploy  │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│       │             │              │             │          │
│  ┌────▼─────────────▼──────────────▼─────────────▼─────┐   │
│  │              Command Layer                           │   │
│  └──────────────────────┬───────────────────────────────┘   │
│                         │                                    │
│  ┌──────────────────────▼───────────────────────────────┐   │
│  │              Core Services                           │   │
│  │  • Config Management                                 │   │
│  │  • Template Engine                                   │   │
│  │  • Sui CLI Wrapper                                   │   │
│  │  • Code Generator                                    │   │
│  └──────────────────────┬───────────────────────────────┘   │
│                         │                                    │
└─────────────────────────┼────────────────────────────────────┘
                          │
                ┌─────────▼──────────┐
                │     Sui CLI        │
                │  (External Tool)   │
                └────────────────────┘
```

## Core Components

### 1. CLI Layer (`src/cli.rs`)

- Parses command-line arguments using `clap`
- Defines all available commands and their options
- Routes commands to appropriate handlers

### 2. Command Layer (`src/commands/`)

Each command is implemented as a separate module:

- **init**: Project scaffolding and initialization
- **build**: Compiles Move contracts
- **test**: Runs Move unit tests
- **deploy**: Publishes contracts to networks
- **generate**: Creates client SDKs
- **node**: Manages local Sui node
- **install**: Plugin installation (Phase 2)

### 3. Configuration (`src/config.rs`)

- Manages `suiforge.config.json` files
- Handles network configurations
- Stores build and deployment settings
- Tracks deployment state in `suiforge.lock.json`

### 4. Template System (`src/templates/`)

- Provides pre-built project templates
- Generates Move code for common patterns
- Supports multiple template types (NFT, Token, DeFi, etc.)
- Extensible for custom templates

### 5. Sui CLI Wrapper (`src/sui.rs`)

- Abstracts Sui CLI commands
- Provides type-safe interfaces
- Handles error translation
- Manages gas objects and addresses

### 6. Code Generation (`src/codegen/`)

- Analyzes Move modules
- Generates TypeScript/Rust/Swift SDKs
- Creates type-safe client libraries
- Handles serialization/deserialization

### 7. Utilities (`src/utils.rs`)

- Progress indicators
- Colored output
- Error formatting
- User feedback

## Data Flow

### Project Initialization

```
User Command
    ↓
CLI Parser
    ↓
Init Command
    ↓
Template Selection
    ↓
File Generation
    ↓
Git Initialization
    ↓
Success Message
```

### Build & Deploy

```
User Command
    ↓
CLI Parser
    ↓
Config Loader
    ↓
Sui CLI Wrapper
    ↓
Build/Publish
    ↓
Lock File Update
    ↓
Success Message
```

### SDK Generation

```
User Command
    ↓
CLI Parser
    ↓
Module Analyzer
    ↓
Code Generator
    ↓
File Writer
    ↓
Package.json/Cargo.toml
    ↓
Success Message
```

## Design Decisions

### Why Rust?

1. **Performance**: Fast compilation and execution
2. **Type Safety**: Catch errors at compile time
3. **Ecosystem**: Great CLI libraries (clap, tokio)
4. **Portability**: Single binary distribution
5. **Blockchain Native**: Aligns with Sui's tech stack

### Why Wrap Sui CLI?

Rather than reimplementing Sui functionality:

1. **Stability**: Leverage tested Sui CLI code
2. **Compatibility**: Always compatible with latest Sui
3. **Maintenance**: Less code to maintain
4. **Focus**: Focus on DX improvements, not core functionality

### Configuration Format

JSON was chosen for configuration because:

1. **Familiarity**: Developers know JSON
2. **Tooling**: Great editor support
3. **Flexibility**: Easy to extend
4. **Parsing**: Fast and reliable

### Template System

Templates are code-based (not file-based) because:

1. **Simplicity**: No external dependencies
2. **Versioning**: Templates version with CLI
3. **Customization**: Easy to modify in code
4. **Type Safety**: Compile-time validation

## Module Library Architecture

The SuiForge modules library (`modules/`) provides reusable Move components:

```
modules/
├── Move.toml
├── sources/
│   ├── access_control.move
│   ├── pausable.move
│   ├── ownable.move
│   ├── vault.move
│   ├── escrow.move
│   └── payment_splitter.move
└── tests/
```

### Module Design Principles

1. **Composability**: Modules work together
2. **Gas Efficiency**: Optimized for low gas costs
3. **Security**: Audited patterns
4. **Simplicity**: Easy to understand and use
5. **Flexibility**: Customizable for different use cases

## Error Handling

SuiForge uses a custom error type (`SuiForgeError`) that:

1. Wraps underlying errors (IO, Serde, etc.)
2. Provides context-specific error messages
3. Uses `thiserror` for ergonomic error handling
4. Displays user-friendly error messages

## Testing Strategy

### Unit Tests

- Test individual functions
- Mock external dependencies
- Fast execution

### Integration Tests

- Test complete workflows
- Use temporary directories
- Verify file generation

### Move Tests

- Test module functionality
- Verify security properties
- Check gas costs

## Future Architecture

### Phase 2: Plugin System

```
┌─────────────────────────────────────┐
│         Plugin Registry             │
├─────────────────────────────────────┤
│  • Plugin Discovery                 │
│  • Version Management               │
│  • Dependency Resolution            │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│         Plugin Loader               │
├─────────────────────────────────────┤
│  • Dynamic Loading                  │
│  • Sandboxing                       │
│  • API Versioning                   │
└─────────────────────────────────────┘
```

### Extensibility Points

1. **Custom Templates**: Add new project templates
2. **Code Generators**: Support new languages
3. **Build Hooks**: Pre/post build scripts
4. **Deployment Strategies**: Custom deployment logic
5. **Testing Frameworks**: Alternative test runners

## Performance Considerations

1. **Lazy Loading**: Load modules only when needed
2. **Caching**: Cache build artifacts
3. **Parallel Execution**: Run tests in parallel
4. **Incremental Builds**: Only rebuild changed modules

## Security Considerations

1. **Input Validation**: Validate all user inputs
2. **Safe Defaults**: Secure by default
3. **Dependency Auditing**: Regular security audits
4. **Sandboxing**: Isolate plugin execution
5. **Gas Limits**: Prevent accidental overspending

## Deployment

### Distribution

- **Cargo**: `cargo install suiforge`
- **Homebrew**: `brew install suiforge` (future)
- **Binary Releases**: GitHub releases
- **Docker**: Containerized version (future)

### Versioning

- Semantic versioning (MAJOR.MINOR.PATCH)
- Changelog for all releases
- Backward compatibility guarantees

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for architecture contribution guidelines.

---

This architecture is designed to be:
- **Modular**: Easy to extend and modify
- **Maintainable**: Clear separation of concerns
- **Testable**: Comprehensive test coverage
- **Performant**: Fast and efficient
- **User-Friendly**: Great developer experience
