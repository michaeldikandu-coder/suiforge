# SuiForge Publishing Guide

Complete guide to make SuiForge available for public use.

## 📋 Pre-Publishing Checklist

### 1. Code Quality
- [ ] All code compiles without errors
- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Code is formatted: `cargo fmt`
- [ ] No security vulnerabilities: `cargo audit`

### 2. Documentation
- [ ] README is complete and accurate
- [ ] All links work
- [ ] Examples are tested
- [ ] Installation instructions are clear

### 3. Legal
- [ ] License file is present (MIT)
- [ ] Copyright notices are correct
- [ ] No proprietary code included

## 🌐 Step 1: GitHub Repository Setup

### Create Repository

1. Go to https://github.com/new
2. Repository name: `suiforge`
3. Description: "A modern developer framework for Sui blockchain smart contracts"
4. Public repository
5. Don't initialize with README (we have one)

### Push Code to GitHub

```bash
# Initialize git (if not already done)
git init

# Add all files
git add .

# Commit
git commit -m "Initial release of SuiForge v0.1.0"

# Add remote (replace YOUR_USERNAME)
git remote add origin https://github.com/YOUR_USERNAME/suiforge.git

# Push to GitHub
git branch -M main
git push -u origin main
```

### Configure Repository Settings

1. Go to repository Settings
2. Enable Issues
3. Enable Discussions (for community Q&A)
4. Add topics: `sui`, `blockchain`, `move`, `smart-contracts`, `cli`, `rust`
5. Add description and website URL

### Create Release

1. Go to Releases → Create a new release
2. Tag: `v0.1.0`
3. Title: `SuiForge v0.1.0 - Initial Release`
4. Description: Copy from CHANGELOG.md
5. Attach binaries (optional, see Step 4)
6. Publish release

## 📦 Step 2: Publish to Crates.io

### Prerequisites

```bash
# Create crates.io account at https://crates.io
# Get API token from https://crates.io/me

# Login to crates.io
cargo login YOUR_API_TOKEN
```

### Update Cargo.toml

Make sure these fields are correct:

```toml
[package]
name = "suiforge"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A modern developer framework for Sui blockchain smart contracts"
license = "MIT"
repository = "https://github.com/YOUR_USERNAME/suiforge"
homepage = "https://github.com/YOUR_USERNAME/suiforge"
documentation = "https://github.com/YOUR_USERNAME/suiforge"
keywords = ["sui", "blockchain", "move", "smart-contracts", "cli"]
categories = ["command-line-utilities", "development-tools"]
readme = "README.md"
```

### Publish

```bash
# Dry run first
cargo publish --dry-run

# If everything looks good, publish
cargo publish
```

### Verify Publication

```bash
# Wait a few minutes, then test installation
cargo install suiforge

# Verify it works
suiforge --version
```

## 🔧 Step 3: Set Up CI/CD

### Create GitHub Actions Workflow

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable]
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: ${{ matrix.rust }}
          override: true
          components: rustfmt, clippy
      
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Check formatting
        run: cargo fmt -- --check
      
      - name: Run clippy
        run: cargo clippy -- -D warnings
      
      - name: Build
        run: cargo build --verbose
      
      - name: Run tests
        run: cargo test --verbose

  release:
    name: Release
    runs-on: ${{ matrix.os }}
    if: startsWith(github.ref, 'refs/tags/')
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: windows-latest
            target: x86_64-pc-windows-msvc
          - os: macos-latest
            target: x86_64-apple-darwin
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          target: ${{ matrix.target }}
          override: true
      
      - name: Build release
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Create archive
        shell: bash
        run: |
          if [ "${{ matrix.os }}" = "windows-latest" ]; then
            7z a suiforge-${{ matrix.target }}.zip ./target/${{ matrix.target }}/release/suiforge.exe
          else
            tar czf suiforge-${{ matrix.target }}.tar.gz -C ./target/${{ matrix.target }}/release suiforge
          fi
      
      - name: Upload Release Asset
        uses: softprops/action-gh-release@v1
        with:
          files: suiforge-*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Create Release Workflow

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
      
      - name: Publish
        run: cargo publish --token ${{ secrets.CARGO_TOKEN }}
```

Add `CARGO_TOKEN` to GitHub Secrets:
1. Go to repository Settings → Secrets → Actions
2. Add new secret: `CARGO_TOKEN` with your crates.io token

## 📱 Step 4: Build Binaries for Distribution

### Build for Multiple Platforms

```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Windows (from Linux with cross)
cargo install cross
cross build --release --target x86_64-pc-windows-gnu

# macOS (from macOS)
cargo build --release --target x86_64-apple-darwin
```

### Create Installation Scripts

Update `install.sh` with actual GitHub URL:

```bash
#!/bin/bash
# Quick install script

REPO="YOUR_USERNAME/suiforge"
LATEST=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep tag_name | cut -d '"' -f 4)

echo "Installing SuiForge $LATEST..."

# Detect OS
OS=$(uname -s)
case "$OS" in
    Linux*)     PLATFORM="x86_64-unknown-linux-gnu";;
    Darwin*)    PLATFORM="x86_64-apple-darwin";;
    *)          echo "Unsupported OS: $OS"; exit 1;;
esac

# Download and install
curl -L "https://github.com/$REPO/releases/download/$LATEST/suiforge-$PLATFORM.tar.gz" | tar xz
sudo mv suiforge /usr/local/bin/
echo "SuiForge installed successfully!"
```

## 🌍 Step 5: Create Website (Optional)

### Option 1: GitHub Pages

Create `docs/index.html`:

```html
<!DOCTYPE html>
<html>
<head>
    <title>SuiForge - Modern Sui Development Framework</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body { font-family: system-ui; max-width: 800px; margin: 0 auto; padding: 20px; }
        code { background: #f4f4f4; padding: 2px 6px; border-radius: 3px; }
        pre { background: #f4f4f4; padding: 15px; border-radius: 5px; overflow-x: auto; }
    </style>
</head>
<body>
    <h1>🔨 SuiForge</h1>
    <p>A modern developer framework for Sui blockchain smart contracts</p>
    
    <h2>Quick Start</h2>
    <pre><code>cargo install suiforge
suiforge init my-project --template nft
cd my-project
suiforge build
suiforge deploy devnet</code></pre>
    
    <h2>Documentation</h2>
    <ul>
        <li><a href="https://github.com/YOUR_USERNAME/suiforge">GitHub Repository</a></li>
        <li><a href="https://github.com/YOUR_USERNAME/suiforge/blob/main/README.md">Full Documentation</a></li>
        <li><a href="https://github.com/YOUR_USERNAME/suiforge/blob/main/QUICKSTART.md">Quick Start Guide</a></li>
    </ul>
</body>
</html>
```

Enable GitHub Pages in repository settings.

### Option 2: Use mdBook

```bash
# Install mdBook
cargo install mdbook

# Create book
mdbook init docs
# Add your documentation
mdbook build
mdbook serve
```

## 💬 Step 6: Community Setup

### Discord Server

1. Create Discord server: https://discord.com/
2. Create channels:
   - #announcements
   - #general
   - #help
   - #showcase
   - #development
3. Add bot for GitHub notifications
4. Update README with Discord link

### Twitter/X Account

1. Create @suiforge account
2. Tweet announcement
3. Share updates and examples
4. Engage with Sui community

### Community Guidelines

Create `CODE_OF_CONDUCT.md`:

```markdown
# Code of Conduct

## Our Pledge

We pledge to make participation in our community a harassment-free experience for everyone.

## Our Standards

- Be respectful and inclusive
- Accept constructive criticism
- Focus on what's best for the community
- Show empathy towards others

## Enforcement

Report violations to [email]
```

## 📢 Step 7: Announce and Promote

### Announcement Checklist

- [ ] Tweet from @suiforge
- [ ] Post in Sui Discord
- [ ] Post on Reddit (r/sui, r/rust)
- [ ] Post on Hacker News
- [ ] Email Sui Foundation
- [ ] Write blog post
- [ ] Create demo video

### Announcement Template

```markdown
🚀 Introducing SuiForge v0.1.0

A modern developer framework for Sui blockchain - think Hardhat for Ethereum, 
but built specifically for Sui Move.

✨ Features:
• One-command project scaffolding
• 6 production-ready templates
• Automated build & deployment
• TypeScript SDK generation
• Reusable Move modules

🔗 Get started: cargo install suiforge
📚 Docs: github.com/YOUR_USERNAME/suiforge

Built with ❤️ for the Sui community
```

## 📊 Step 8: Monitor and Maintain

### Set Up Analytics

- GitHub Stars and Forks
- Crates.io download stats
- Discord member count
- Issue response time

### Maintenance Tasks

Weekly:
- [ ] Respond to issues
- [ ] Review pull requests
- [ ] Update dependencies
- [ ] Check for security issues

Monthly:
- [ ] Review roadmap
- [ ] Plan new features
- [ ] Update documentation
- [ ] Community engagement

## 🎯 Success Metrics

Track these metrics:

- **Downloads**: Crates.io downloads
- **Stars**: GitHub stars
- **Community**: Discord members
- **Adoption**: Projects using SuiForge
- **Contributions**: Pull requests and contributors

## 📝 Post-Launch Checklist

### Week 1
- [ ] Monitor for critical bugs
- [ ] Respond to all issues within 24h
- [ ] Gather initial feedback
- [ ] Fix urgent issues

### Month 1
- [ ] Release patch version if needed
- [ ] Write tutorial blog posts
- [ ] Create video tutorials
- [ ] Engage with early adopters

### Month 3
- [ ] Plan v0.2.0 features
- [ ] Implement community requests
- [ ] Improve documentation
- [ ] Grow community

## 🚨 Emergency Response

If critical bug found:

1. Acknowledge immediately
2. Create hotfix branch
3. Fix and test thoroughly
4. Release patch version
5. Yank broken version from crates.io: `cargo yank --vers 0.1.0`
6. Publish fixed version
7. Announce fix

## 📞 Support Channels

Set up:
- GitHub Issues for bugs
- GitHub Discussions for questions
- Discord for real-time help
- Email for private matters

## 🎉 You're Ready!

Once you complete these steps, SuiForge will be:
- ✅ Available on crates.io
- ✅ Documented on GitHub
- ✅ Tested with CI/CD
- ✅ Supported by community
- ✅ Ready for users!

---

Good luck with your launch! 🚀
