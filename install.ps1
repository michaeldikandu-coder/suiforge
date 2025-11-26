# SuiForge Installation Script for Windows
# Run with: powershell -ExecutionPolicy Bypass -File install.ps1

Write-Host "🔨 SuiForge Installation Script" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
$rustInstalled = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $rustInstalled) {
    Write-Host "❌ Rust is not installed." -ForegroundColor Red
    Write-Host "📦 Please install Rust from: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "   Then run this script again." -ForegroundColor Yellow
    exit 1
} else {
    Write-Host "✅ Rust is already installed" -ForegroundColor Green
}

# Check if Sui CLI is installed
$suiInstalled = Get-Command sui -ErrorAction SilentlyContinue
if (-not $suiInstalled) {
    Write-Host "❌ Sui CLI is not installed." -ForegroundColor Red
    Write-Host "📦 Installing Sui CLI (this may take a while)..." -ForegroundColor Yellow
    cargo install --git https://github.com/MystenLabs/sui.git sui
    Write-Host "✅ Sui CLI installed successfully" -ForegroundColor Green
} else {
    Write-Host "✅ Sui CLI is already installed" -ForegroundColor Green
}

# Install SuiForge
Write-Host ""
Write-Host "📦 Installing SuiForge..." -ForegroundColor Yellow

if (Test-Path ".git") {
    # Installing from source
    Write-Host "Installing from source..." -ForegroundColor Cyan
    cargo install --path .
} else {
    # Installing from cargo (when published)
    Write-Host "Installing from cargo..." -ForegroundColor Cyan
    cargo install suiforge
}

Write-Host ""
Write-Host "✅ SuiForge installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "🚀 Quick Start:" -ForegroundColor Cyan
Write-Host "  suiforge init my-project --template nft"
Write-Host "  cd my-project"
Write-Host "  suiforge build"
Write-Host "  suiforge test"
Write-Host "  suiforge deploy devnet"
Write-Host ""
Write-Host "📚 Documentation: https://suiforge.dev" -ForegroundColor Yellow
Write-Host "💬 Discord: https://discord.gg/suiforge" -ForegroundColor Yellow
Write-Host "🐙 GitHub: https://github.com/yourusername/suiforge" -ForegroundColor Yellow
Write-Host ""
Write-Host "Happy building! 🎉" -ForegroundColor Magenta
