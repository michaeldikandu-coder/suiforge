# Contributing to SuiForge

Thank you for your interest in contributing to SuiForge! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/suiforge.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add new feature'`
7. Push to your fork: `git push origin feature/your-feature-name`
8. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Sui CLI installed
- Git

### Building from Source

```bash
cargo build
```

### Running Tests

```bash
cargo test
cargo test --all-features
```

### Running the CLI Locally

```bash
cargo run -- init test-project
```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy for linting: `cargo clippy`
- Write clear, descriptive commit messages
- Add tests for new features
- Update documentation as needed

## Project Structure

```
suiforge/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI argument parsing
│   ├── commands/         # Command implementations
│   ├── config.rs         # Configuration management
│   ├── sui.rs            # Sui CLI wrapper
│   ├── templates/        # Project templates
│   ├── codegen/          # SDK generators
│   └── utils.rs          # Utility functions
├── modules/              # SuiForge Move modules library
│   └── sources/          # Move source files
└── tests/                # Integration tests
```

## Adding New Commands

1. Create a new file in `src/commands/`
2. Implement the `execute` function
3. Add the command to `src/cli.rs`
4. Add the command handler in `src/main.rs`
5. Write tests
6. Update documentation

Example:

```rust
// src/commands/mycommand.rs
use crate::error::Result;

pub async fn execute() -> Result<()> {
    // Implementation
    Ok(())
}
```

## Adding New Templates

1. Create a new file in `src/templates/`
2. Implement the template generation function
3. Add the template to `Template` enum in `src/templates/mod.rs`
4. Add template-specific Move code
5. Test the template generation

## Adding New Move Modules

1. Create a new `.move` file in `modules/sources/`
2. Follow Sui Move best practices
3. Add comprehensive tests
4. Document all public functions
5. Update `modules/README.md`

### Move Module Guidelines

- Use clear, descriptive names
- Include error codes with descriptive constants
- Add inline documentation
- Write unit tests
- Consider gas optimization
- Follow the Sui object model

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
cargo test --test integration
```

### Move Module Tests

```bash
cd modules
sui move test
```

## Documentation

- Update README.md for user-facing changes
- Add inline code documentation
- Update CHANGELOG.md
- Include examples for new features

## Pull Request Process

1. Ensure all tests pass
2. Update documentation
3. Add entry to CHANGELOG.md
4. Request review from maintainers
5. Address review feedback
6. Squash commits if requested

## Code Review Guidelines

- Be respectful and constructive
- Focus on code quality and maintainability
- Suggest improvements, don't demand them
- Approve when ready, request changes when needed

## Reporting Issues

When reporting issues, please include:

- SuiForge version
- Operating system
- Sui CLI version
- Steps to reproduce
- Expected vs actual behavior
- Error messages or logs

## Feature Requests

We welcome feature requests! Please:

- Check if the feature already exists
- Describe the use case
- Explain why it would be valuable
- Consider implementation complexity

## Community

- Discord: https://discord.gg/suiforge
- Twitter: @suiforge
- GitHub Discussions: For questions and ideas

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to open an issue or reach out on Discord!

---

Thank you for contributing to SuiForge! 🚀
