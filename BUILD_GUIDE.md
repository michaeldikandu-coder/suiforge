# SuiForge Build Guide

Complete guide for building, testing, and deploying SuiForge.

## Prerequisites

### Required

- **Rust 1.70+**: Install from https://rustup.rs/
- **Git**: For version control
- **Sui CLI**: For Move compilation and deployment

### Optional

- **Make**: For using Makefile commands
- **Docker**: For containerized builds (future)

## Installation

### Quick Install (Linux/macOS)

```bash
curl -sSf https://raw.githubusercontent.com/yourusername/suiforge/main/install.sh | bash
```

### Quick Install (Windows)

```powershell
iwr https://raw.githubusercontent.com/yourusername/suiforge/main/install.ps1 -useb | iex
```

### Manual Installation

#### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 2. Install Sui CLI

```bash
cargo install --git https://github.com/MystenLabs/sui.git sui
```

#### 3. Clone SuiForge

```bash
git clone https://github.com/yourusername/suiforge.git
cd suiforge
```

#### 4. Build from Source

```bash
cargo build --release
```

#### 5. Install Locally

```bash
cargo install --path .
```

#### 6. Verify Installation

```bash
suiforge --version
```

## Development Setup

### Clone and Setup

```bash
# Clone repository
git clone https://github.com/yourusername/suiforge.git
cd suiforge

# Install dependencies (automatically handled by cargo)
cargo build
```

### Project Structure

```
suiforge/
├── src/                    # Rust source code
│   ├── main.rs            # Entry point
│   ├── cli.rs             # CLI definitions
│   ├── commands/          # Command implementations
│   ├── templates/         # Project templates
│   ├── codegen/           # SDK generators
│   └── ...
├── modules/               # Move modules library
│   ├── sources/           # Move source files
│   └── Move.toml
├── examples/              # Usage examples
├── tests/                 # Integration tests
├── Cargo.toml            # Rust dependencies
└── ...
```

## Building

### Development Build

```bash
# Fast build for development
cargo build

# Or using Make
make dev-build
```

### Release Build

```bash
# Optimized build for production
cargo build --release

# Or using Make
make build
```

### Build Output

- Debug: `target/debug/suiforge`
- Release: `target/release/suiforge`

## Testing

### Run All Tests

```bash
cargo test

# Or using Make
make test
```

### Run Specific Tests

```bash
# Test a specific module
cargo test config

# Test with output
cargo test -- --nocapture

# Test with specific features
cargo test --all-features
```

### Test Move Modules

```bash
cd modules
sui move test
```

## Code Quality

### Format Code

```bash
cargo fmt --all

# Or using Make
make fmt
```

### Run Linter

```bash
cargo clippy --all-targets --all-features -- -D warnings

# Or using Make
make clippy
```

### Check Everything

```bash
# Run all checks before committing
make check
```

This runs:
1. Code formatting
2. Clippy linting
3. All tests

## Running Locally

### Run from Source

```bash
# Run without installing
cargo run -- --help
cargo run -- init test-project
cargo run -- build

# Or using Make
make dev-run
```

### Install Locally

```bash
# Install to ~/.cargo/bin/
cargo install --path .

# Now use globally
suiforge --help
```

## Documentation

### Generate Rust Docs

```bash
cargo doc --no-deps --open

# Or using Make
make doc
```

### View Documentation

Documentation will open in your browser at:
`target/doc/suiforge/index.html`

## Debugging

### Enable Debug Logging

```bash
# Set log level
export RUST_LOG=debug
suiforge build

# Or inline
RUST_LOG=debug suiforge build
```

### Debug Build

```bash
# Build with debug symbols
cargo build

# Run with debugger (lldb on macOS, gdb on Linux)
lldb target/debug/suiforge
```

### Common Issues

#### Build Fails

```bash
# Clean and rebuild
cargo clean
cargo build
```

#### Dependency Issues

```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

#### Test Failures

```bash
# Run tests with backtrace
RUST_BACKTRACE=1 cargo test
```

## Performance Profiling

### Build Time

```bash
# Measure build time
cargo clean
time cargo build --release
```

### Runtime Performance

```bash
# Profile with perf (Linux)
perf record target/release/suiforge build
perf report

# Profile with Instruments (macOS)
instruments -t "Time Profiler" target/release/suiforge
```

## Cross-Compilation

### Linux to Windows

```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

### macOS to Linux

```bash
# Install target
rustup target add x86_64-unknown-linux-gnu

# Build (requires cross-compilation tools)
cargo build --release --target x86_64-unknown-linux-gnu
```

## Release Process

### 1. Update Version

Edit `Cargo.toml`:
```toml
[package]
version = "0.2.0"
```

### 2. Update Changelog

Add release notes to `CHANGELOG.md`

### 3. Build Release

```bash
make release
```

### 4. Test Release

```bash
target/release/suiforge --version
target/release/suiforge init test-project
```

### 5. Create Git Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### 6. Publish to Cargo

```bash
cargo publish
```

## Continuous Integration

### GitHub Actions

`.github/workflows/ci.yml`:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --verbose
      - run: cargo test --verbose
      - run: cargo clippy -- -D warnings
```

### Local CI Simulation

```bash
# Run the same checks as CI
make check
```

## Docker Build (Future)

### Build Docker Image

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/suiforge /usr/local/bin/
CMD ["suiforge"]
```

```bash
docker build -t suiforge .
docker run suiforge --version
```

## Troubleshooting

### Rust Version Issues

```bash
# Update Rust
rustup update stable

# Check version
rustc --version
```

### Cargo Cache Issues

```bash
# Clear cargo cache
rm -rf ~/.cargo/registry
rm -rf ~/.cargo/git

# Rebuild
cargo build
```

### Linker Errors

```bash
# Install build essentials (Ubuntu/Debian)
sudo apt-get install build-essential

# Install Xcode Command Line Tools (macOS)
xcode-select --install
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Code style guidelines
- Testing requirements
- Pull request process
- Development workflow

## Getting Help

- **GitHub Issues**: Report bugs or build problems
- **Discord**: Ask for help from the community
- **Documentation**: Check the full docs at https://suiforge.dev

## Next Steps

1. **Build the project**: `cargo build --release`
2. **Run tests**: `cargo test`
3. **Install locally**: `cargo install --path .`
4. **Try it out**: `suiforge init test-project`
5. **Read the docs**: See [README.md](README.md)

---

Happy building! 🚀
