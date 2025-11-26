#!/bin/bash

# SuiForge Installation Script
# This script installs SuiForge and its dependencies

set -e

echo "🔨 SuiForge Installation Script"
echo "================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed."
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed successfully"
else
    echo "✅ Rust is already installed"
fi

# Check if Sui CLI is installed
if ! command -v sui &> /dev/null; then
    echo "❌ Sui CLI is not installed."
    echo "📦 Installing Sui CLI (this may take a while)..."
    cargo install --git https://github.com/MystenLabs/sui.git sui
    echo "✅ Sui CLI installed successfully"
else
    echo "✅ Sui CLI is already installed"
fi

# Install SuiForge
echo ""
echo "📦 Installing SuiForge..."

if [ -d ".git" ]; then
    # Installing from source
    echo "Installing from source..."
    cargo install --path .
else
    # Installing from cargo (when published)
    echo "Installing from cargo..."
    cargo install suiforge
fi

echo ""
echo "✅ SuiForge installed successfully!"
echo ""
echo "🚀 Quick Start:"
echo "  suiforge init my-project --template nft"
echo "  cd my-project"
echo "  suiforge build"
echo "  suiforge test"
echo "  suiforge deploy devnet"
echo ""
echo "📚 Documentation: https://suiforge.dev"
echo "💬 Discord: https://discord.gg/suiforge"
echo "🐙 GitHub: https://github.com/yourusername/suiforge"
echo ""
echo "Happy building! 🎉"
