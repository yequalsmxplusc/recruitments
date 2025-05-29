#!/bin/bash

set -e  # Exit on any error

# Install Rust if not present
curl https://sh.rustup.rs -sSf | sh -s -- -y

source "$HOME/.cargo/env"

# Ensure Cargo is available in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Set default Rust toolchain
rustup default stable

# Install Trunk (if not already installed)
cargo install trunk

# Optional: install wasm32 target if needed
rustup target add wasm32-unknown-unknown

# Build your Yew app
trunk build --release --public-url "/" --config Trunk.toml