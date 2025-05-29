#!/bin/bash
set -e  # Exit immediately on error

# Ensure we’re in the correct directory (where Cargo.toml and Trunk.toml are)
cd "$(dirname "$0")"

echo "🔧 Installing Rust (if missing)..."
if ! command -v rustup &> /dev/null; then
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source "$HOME/.cargo/env"
fi

export PATH="$HOME/.cargo/bin:$PATH"

echo "📦 Setting Rust to stable toolchain..."
rustup default stable

echo "📥 Installing Trunk (if not already installed)..."
if ! command -v trunk &> /dev/null; then
    cargo install trunk
fi

echo "🎯 Adding wasm32 target for Rust..."
rustup target add wasm32-unknown-unknown

echo "🏗️ Building Yew app with Trunk..."
trunk build --release --public-url "/" --config Trunk.toml

echo "🪄 Fixing SPA refresh issue: Copying index.html → 404.html"
cp dist/index.html dist/404.html

echo "✅ Build complete!"