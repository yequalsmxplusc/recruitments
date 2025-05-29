#!/bin/bash

set -e  # Exit on error

# Navigate to the directory where Cargo.toml, Trunk.toml, and index.html live
cd "$(dirname "$0")"  # This makes sure we are in /frontend

# Install Rust if not present
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
export PATH="$HOME/.cargo/bin:$PATH"

# Set default Rust toolchain
rustup default stable

# Install Trunk (skip if already installed)
cargo install trunk || true

# Add wasm target
rustup target add wasm32-unknown-unknown

# Build the Yew app
trunk build --release --public-url "/" --config Trunk.toml