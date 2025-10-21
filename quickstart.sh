#!/bin/bash
# Garry Quick Start Script

set -e

echo "🚀 Garry Quick Start"
echo "===================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust/Cargo found"

# Build the project
echo ""
echo "📦 Building Garry..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Check if config exists
if [ ! -f ".garry/config.toml" ]; then
    echo ""
    echo "⚠️  Configuration not found"
    echo "   Creating .garry/config.toml from example..."
    mkdir -p .garry
    cp .garry/config.toml.example .garry/config.toml
    echo ""
    echo "📝 Please edit .garry/config.toml with your settings:"
    echo "   - VCS token"
    echo "   - Repository name"
    echo "   - Platform (github, gitlab, etc.)"
    echo ""
    echo "   Then run this script again."
    exit 0
fi

echo "✓ Configuration found"

# Install binaries
echo ""
echo "📥 Installing binaries..."
cargo install --path .

if [ $? -eq 0 ]; then
    echo "✓ Installation successful"
else
    echo "❌ Installation failed"
    exit 1
fi

# Verify installation
echo ""
echo "🔍 Verifying installation..."

if command -v garry &> /dev/null; then
    echo "✓ garry CLI installed"
    garry --version
else
    echo "❌ garry CLI not found in PATH"
    echo "   Add ~/.cargo/bin to your PATH"
    exit 1
fi

if command -v garry-bot &> /dev/null; then
    echo "✓ garry-bot installed"
else
    echo "❌ garry-bot not found in PATH"
    exit 1
fi

# Success!
echo ""
echo "✅ Garry is ready to use!"
echo ""
echo "Quick commands:"
echo "  garry start feature/my-feature  - Start a new feature branch"
echo "  garry squash                    - Squash commits"
echo "  garry upload                    - Upload for review"
echo "  garry update                    - Update existing review"
echo "  garry merge                     - Initiate merge"
echo ""
echo "  garry-bot                       - Start the bot service"
echo ""
echo "Documentation:"
echo "  README.md  - Project overview"
echo "  USAGE.md   - Detailed usage guide"
echo ""
echo "Happy coding! 🎉"
