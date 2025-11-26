# 🚀 SuiForge Launch Checklist

Quick checklist to launch SuiForge and make it available to users.

## ✅ Pre-Launch (Do This First)

### 1. Test Everything
```bash
# Build and test
cargo build --release
cargo test
cargo clippy
cargo fmt --check

# Test the CLI
./target/release/suiforge init test-project --template nft
cd test-project
../target/release/suiforge build
```

- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code is formatted
- [ ] CLI works correctly

### 2. Update Cargo.toml

Replace placeholders in `Cargo.toml`:

```toml
[package]
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/YOUR_USERNAME/suiforge"
homepage = "https://github.com/YOUR_USERNAME/suiforge"
```

- [ ] Author info updated
- [ ] Repository URL updated
- [ ] Version is correct (0.1.0)

### 3. Update Documentation

Update these files with your actual URLs:
- [ ] README.md - Replace placeholder URLs
- [ ] install.sh - Add your GitHub username
- [ ] install.ps1 - Add your GitHub username
- [ ] All documentation files

## 🌐 GitHub Setup (30 minutes)

### 1. Create Repository

1. Go to https://github.com/new
2. Name: `suiforge`
3. Description: "A modern developer framework for Sui blockchain smart contracts"
4. Public repository
5. Don't initialize (we have files)

- [ ] Repository created

### 2. Push Code

```bash
git init
git add .
git commit -m "Initial release of SuiForge v0.1.0"
git remote add origin https://github.com/YOUR_USERNAME/suiforge.git
git branch -M main
git push -u origin main
```

- [ ] Code pushed to GitHub

### 3. Configure Repository

In GitHub repository settings:
- [ ] Enable Issues
- [ ] Enable Discussions
- [ ] Add topics: `sui`, `blockchain`, `move`, `smart-contracts`, `cli`, `rust`
- [ ] Add description
- [ ] Set up branch protection for `main`

### 4. Add Secrets for CI/CD

Go to Settings → Secrets → Actions:
- [ ] Add `CARGO_TOKEN` (get from https://crates.io/me)

## 📦 Publish to Crates.io (15 minutes)

### 1. Create Crates.io Account

- [ ] Sign up at https://crates.io
- [ ] Get API token from https://crates.io/me

### 2. Publish

```bash
# Login
cargo login YOUR_API_TOKEN

# Dry run
cargo publish --dry-run

# Publish for real
cargo publish
```

- [ ] Published to crates.io
- [ ] Verify at https://crates.io/crates/suiforge

### 3. Test Installation

```bash
# In a different directory
cargo install suiforge
suiforge --version
suiforge init test --template nft
```

- [ ] Installation works
- [ ] CLI works after install

## 🏷️ Create First Release (10 minutes)

### 1. Create Git Tag

```bash
git tag -a v0.1.0 -m "Release v0.1.0 - Initial release"
git push origin v0.1.0
```

- [ ] Tag created and pushed

### 2. Create GitHub Release

1. Go to Releases → Create new release
2. Choose tag: `v0.1.0`
3. Title: `SuiForge v0.1.0 - Initial Release`
4. Description: Copy from CHANGELOG.md
5. Publish

- [ ] GitHub release created
- [ ] Binaries attached (automated by CI)

## 💬 Community Setup (1 hour)

### 1. Discord Server (Optional but Recommended)

1. Create server at https://discord.com
2. Create channels:
   - #announcements
   - #general
   - #help
   - #showcase
3. Update README with Discord link

- [ ] Discord server created
- [ ] Invite link in README

### 2. Social Media (Optional)

- [ ] Create Twitter/X account @suiforge
- [ ] Create first tweet announcing launch
- [ ] Follow Sui-related accounts

## 📢 Announce Launch (30 minutes)

### 1. Write Announcement

Use this template:

```
🚀 Introducing SuiForge v0.1.0

A modern developer framework for Sui blockchain - think Hardhat for 
Ethereum, but built specifically for Sui Move.

✨ Features:
• One-command project scaffolding with 6 templates
• Automated build, test & deployment
• TypeScript SDK generation
• 6 reusable Move modules
• Beautiful CLI with progress indicators

🔗 Install: cargo install suiforge
📚 Docs: github.com/YOUR_USERNAME/suiforge
💬 Discord: [your-discord-link]

Built with ❤️ for the Sui community

#Sui #Blockchain #Move #Web3 #DeveloperTools
```

### 2. Post Announcements

- [ ] Tweet from @suiforge
- [ ] Post in Sui Discord (#dev-resources)
- [ ] Post on Reddit r/sui
- [ ] Post on Reddit r/rust
- [ ] Post on Hacker News (Show HN)
- [ ] Email Sui Foundation/Team
- [ ] Post in relevant Telegram groups

### 3. Sui Community

- [ ] Join Sui Discord
- [ ] Introduce SuiForge in #introductions
- [ ] Share in #dev-resources
- [ ] Engage with community

## 📊 Post-Launch (Ongoing)

### Week 1

- [ ] Monitor GitHub issues daily
- [ ] Respond to questions within 24h
- [ ] Fix critical bugs immediately
- [ ] Gather feedback

### Week 2-4

- [ ] Write tutorial blog post
- [ ] Create demo video
- [ ] Improve documentation based on feedback
- [ ] Plan v0.2.0 features

### Month 2-3

- [ ] Implement community requests
- [ ] Release v0.2.0
- [ ] Grow community
- [ ] Seek partnerships

## 🎯 Success Metrics to Track

- [ ] GitHub stars
- [ ] Crates.io downloads
- [ ] Discord members
- [ ] GitHub issues/PRs
- [ ] Projects using SuiForge

## 🚨 Emergency Contacts

If critical bug found:

1. Acknowledge in GitHub issue
2. Create hotfix branch
3. Fix and test
4. Release patch (v0.1.1)
5. Yank broken version: `cargo yank --vers 0.1.0`
6. Announce fix

## ✅ Launch Complete!

Once all checkboxes are checked, SuiForge is live! 🎉

### You should now have:

✅ Code on GitHub  
✅ Package on crates.io  
✅ CI/CD running  
✅ Community channels  
✅ Public announcement  
✅ Users installing and using SuiForge  

## 📞 Need Help?

- Check [PUBLISHING_GUIDE.md](PUBLISHING_GUIDE.md) for detailed instructions
- Ask in Rust Discord
- Ask in Sui Discord
- Open GitHub Discussion

---

**Estimated Total Time**: 2-3 hours

**Best Time to Launch**: Tuesday-Thursday morning (US time) for maximum visibility

Good luck with your launch! 🚀
