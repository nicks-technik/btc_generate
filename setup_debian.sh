#!/bin/bash
# setup_debian.sh - Prepares a Debian/Ubuntu system for btc_generate

# Exit on any error
set -e

echo "Starting system preparation for btc_generate..."

# 1. Update system package lists
echo "Updating package lists..."
sudo apt-get update -y

# 2. Install essential build dependencies
# build-essential: provides gcc/g++ and make
# pkg-config & libssl-dev: often required by Rust cryptographic crates
# curl & git: for fetching rustup and repository management
echo "Installing build-essential, curl, git, pkg-config, and libssl-dev..."
sudo apt-get install -y build-essential curl git pkg-config libssl-dev

# 3. Install Rust toolchain via rustup
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source the cargo environment for the current shell session
    source "$HOME/.cargo/env"
    echo "Rust installed successfully."
else
    echo "Rust/Cargo is already installed. Skipping installation."
fi

# 4. Verify installations
echo "------------------------------------------"
echo "Verification:"
gcc --version | head -n 1
cargo --version
rustc --version
echo "------------------------------------------"

echo "System preparation complete!"
echo "To start using 'cargo' in this terminal, run:"
echo "source \$HOME/.cargo/env"
echo ""
echo "Then you can build the project with:"
echo "cargo build --release"
