# SuiForge Getting Started Checklist

Complete checklist for setting up and using SuiForge.

## 📋 Pre-Installation Checklist

### System Requirements
- [ ] Operating System: Linux, macOS, or Windows
- [ ] Internet connection for downloading dependencies
- [ ] At least 2GB free disk space
- [ ] Terminal/Command prompt access

### Required Software
- [ ] **Rust** (1.70 or later)
  - Check: `rustc --version`
  - Install: https://rustup.rs/
  
- [ ] **Cargo** (comes with Rust)
  - Check: `cargo --version`
  
- [ ] **Git** (for version control)
  - Check: `git --version`
  - Install: https://git-scm.com/

- [ ] **Sui CLI** (for Move compilation)
  - Check: `sui --version`
  - Install: `cargo install --git https://github.com/MystenLabs/sui.git sui`

### Optional Software
- [ ] **Make** (for using Makefile)
  - Check: `make --version`
  
- [ ] **Code Editor** (VS Code, Vim, etc.)
  - Recommended: VS Code with Move extension

## 🔧 Installation Checklist

### Option 1: Quick Install (Recommended)

#### Linux/macOS
- [ ] Run: `curl -sSf https://raw.githubusercontent.com/yourusername/suiforge/main/install.sh | bash`
- [ ] Verify: `suiforge --version`

#### Windows
- [ ] Run: `iwr https://raw.githubusercontent.com/yourusername/suiforge/main/install.ps1 -useb | iex`
- [ ] Verify: `suiforge --version`

### Option 2: From Cargo
- [ ] Run: `cargo install suiforge`
- [ ] Verify: `suiforge --version`

### Option 3: From Source
- [ ] Clone: `git clone https://github.com/yourusername/suiforge.git`
- [ ] Navigate: `cd suiforge`
- [ ] Build: `cargo build --release`
- [ ] Install: `cargo install --path .`
- [ ] Verify: `suiforge --version`

## 🎯 First Project Checklist

### 1. Create Project
- [ ] Choose a project name
- [ ] Choose a template (basic, nft, token, marketplace, defi, game)
- [ ] Run: `suiforge init <project-name> --template <template>`
- [ ] Navigate: `cd <project-name>`

### 2. Explore Project Structure
- [ ] Review `Move.toml`
- [ ] Review `suiforge.config.json`
- [ ] Read `README.md`
- [ ] Examine `sources/main.move`
- [ ] Check `tests/main_tests.move`

### 3. Build Project
- [ ] Run: `suiforge build`
- [ ] Check for errors
- [ ] Review build output in `build/` directory

### 4. Run Tests
- [ ] Run: `suiforge test`
- [ ] Verify all tests pass
- [ ] Review test output

### 5. Configure Sui
- [ ] Run: `sui client`
- [ ] Set up wallet if needed
- [ ] Select network (devnet recommended for testing)
- [ ] Get devnet tokens from faucet

### 6. Deploy to Devnet
- [ ] Run: `suiforge deploy devnet`
- [ ] Note the Package ID
- [ ] Check `suiforge.lock.json` for deployment info
- [ ] Verify deployment on Sui Explorer

### 7. Generate SDK
- [ ] Run: `suiforge generate ts`
- [ ] Check `sdk/typescript/` directory
- [ ] Review generated files
- [ ] Read SDK README

## 📚 Learning Checklist

### Documentation
- [ ] Read [README.md](README.md)
- [ ] Read [QUICKSTART.md](QUICKSTART.md)
- [ ] Browse [examples/basic-usage.md](examples/basic-usage.md)
- [ ] Review [VISUAL_GUIDE.md](VISUAL_GUIDE.md)

### Commands
- [ ] Try `suiforge --help`
- [ ] Try `suiforge init --help`
- [ ] Try `suiforge build --help`
- [ ] Try `suiforge deploy --help`

### Templates
- [ ] Create a basic project
- [ ] Create an NFT project
- [ ] Create a token project
- [ ] Explore other templates

### Move Modules
- [ ] Read [modules/README.md](modules/README.md)
- [ ] Review module source code
- [ ] Try using a module in your project

## 🚀 Advanced Checklist

### Local Development
- [ ] Start local node: `suiforge node start`
- [ ] Check node status: `suiforge node status`
- [ ] Deploy to local node
- [ ] Stop node: `suiforge node stop`

### Configuration
- [ ] Customize `suiforge.config.json`
- [ ] Set custom gas budget
- [ ] Configure SDK output directories
- [ ] Set up custom network

### SDK Integration
- [ ] Install generated SDK in frontend
- [ ] Import SDK in your code
- [ ] Create transactions using SDK
- [ ] Test SDK functionality

### Multiple Networks
- [ ] Deploy to devnet
- [ ] Deploy to testnet
- [ ] (Optional) Deploy to mainnet

### CI/CD
- [ ] Set up GitHub Actions
- [ ] Add build step
- [ ] Add test step
- [ ] Add deployment step

## 🔍 Troubleshooting Checklist

### Build Issues
- [ ] Check Rust version: `rustc --version`
- [ ] Check Sui CLI: `sui --version`
- [ ] Clean build: `rm -rf build/`
- [ ] Rebuild: `suiforge build`

### Test Issues
- [ ] Review test code
- [ ] Check for syntax errors
- [ ] Run with filter: `suiforge test --filter <test_name>`
- [ ] Check test output

### Deployment Issues
- [ ] Check active address: `sui client active-address`
- [ ] Check gas objects: `sui client gas`
- [ ] Get devnet tokens from faucet
- [ ] Increase gas budget: `suiforge deploy devnet --gas-budget 100000000`

### SDK Issues
- [ ] Verify deployment succeeded
- [ ] Check Package ID in `suiforge.lock.json`
- [ ] Regenerate SDK: `suiforge generate ts`
- [ ] Check SDK dependencies

## 🎓 Best Practices Checklist

### Development
- [ ] Use version control (Git)
- [ ] Write comprehensive tests
- [ ] Follow Move best practices
- [ ] Use SuiForge modules when possible
- [ ] Keep dependencies updated

### Testing
- [ ] Test locally before deploying
- [ ] Test on devnet before testnet
- [ ] Test on testnet before mainnet
- [ ] Write unit tests for all functions
- [ ] Test edge cases

### Deployment
- [ ] Review code before deploying
- [ ] Test thoroughly on devnet
- [ ] Use appropriate gas budget
- [ ] Save deployment info
- [ ] Verify deployment

### Documentation
- [ ] Document your code
- [ ] Update README
- [ ] Add usage examples
- [ ] Document deployment process

## 🤝 Community Checklist

### Get Involved
- [ ] Join Discord: https://discord.gg/suiforge
- [ ] Follow on Twitter: @suiforge
- [ ] Star on GitHub
- [ ] Read contribution guidelines

### Contribute
- [ ] Report bugs
- [ ] Suggest features
- [ ] Submit pull requests
- [ ] Help other users
- [ ] Write tutorials

### Share
- [ ] Share your projects
- [ ] Write blog posts
- [ ] Create tutorials
- [ ] Give feedback

## ✅ Completion Checklist

### You're Ready When You Can:
- [ ] Create a new project
- [ ] Build and test contracts
- [ ] Deploy to devnet
- [ ] Generate and use SDKs
- [ ] Use SuiForge modules
- [ ] Troubleshoot common issues
- [ ] Find help when needed

### Next Steps:
- [ ] Build a real project
- [ ] Explore advanced features
- [ ] Contribute to SuiForge
- [ ] Help other developers
- [ ] Share your experience

## 📊 Progress Tracker

Track your progress:

```
Installation:        [ ] Not Started  [ ] In Progress  [ ] Complete
First Project:       [ ] Not Started  [ ] In Progress  [ ] Complete
Learning:            [ ] Not Started  [ ] In Progress  [ ] Complete
Advanced Features:   [ ] Not Started  [ ] In Progress  [ ] Complete
Best Practices:      [ ] Not Started  [ ] In Progress  [ ] Complete
Community:           [ ] Not Started  [ ] In Progress  [ ] Complete
```

## 🎉 Congratulations!

Once you've completed this checklist, you're ready to build amazing applications on Sui with SuiForge!

## 📞 Need Help?

- **Documentation**: Check [DOCUMENTATION_INDEX.md](DOCUMENTATION_INDEX.md)
- **Examples**: See [examples/basic-usage.md](examples/basic-usage.md)
- **Issues**: https://github.com/yourusername/suiforge/issues
- **Discord**: https://discord.gg/suiforge
- **Twitter**: @suiforge

---

Happy building with SuiForge! 🚀
