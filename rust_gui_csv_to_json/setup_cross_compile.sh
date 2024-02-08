#!/bin/bash

# Stop script on any error
set -e

# Step 1: Add Rust target for Windows
echo "Adding Rust target for x86_64 Windows..."
rustup target add x86_64-pc-windows-gnu

# Step 2: Install MinGW-w64
echo "Installing MinGW-w64..."
sudo apt update
sudo apt install -y mingw-w64

# Step 3: Configure Cargo to use MinGW-w64 as linker
echo "Configuring Cargo to use MinGW-w64 linker for x86_64-pc-windows-gnu target..."

# Define Cargo config path
CARGO_CONFIG="$HOME/.cargo/config"

# Ensure .cargo directory exists
mkdir -p "$(dirname "$CARGO_CONFIG")"

# Check if the config file already exists
if [ ! -f "$CARGO_CONFIG" ]; then
    # Create a new Cargo config file if it doesn't exist
    echo "[target.x86_64-pc-windows-gnu]" >> "$CARGO_CONFIG"
    echo "linker = \"x86_64-w64-mingw32-gcc\"" >> "$CARGO_CONFIG"
else
    # Append to the existing Cargo config file if the target config doesn't exist
    if ! grep -q "[target.x86_64-pc-windows-gnu]" "$CARGO_CONFIG"; then
        echo "Adding new target configuration to existing Cargo config..."
        echo "" >> "$CARGO_CONFIG"
        echo "[target.x86_64-pc-windows-gnu]" >> "$CARGO_CONFIG"
        echo "linker = \"x86_64-w64-mingw32-gcc\"" >> "$CARGO_CONFIG"
    else
        echo "Configuration for x86_64-pc-windows-gnu already exists in Cargo config."
    fi
fi

echo "Setup complete! You can now cross-compile your Rust projects for Windows by using:"
echo "cargo build --target x86_64=pc-windows-gun --release"
