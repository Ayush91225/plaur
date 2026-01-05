#!/bin/bash

set -e

echo "🚀 Starting Plaur AI-First Commerce Intelligence Platform"
echo "📍 Running on localhost"
echo ""

# Set environment for bootstrap mode
export PLAUR_SERVICE_TYPE=bootstrap
export PLAUR_ENV=development
export RUST_LOG=info

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first."
    exit 1
fi

echo "🔧 Building Plaur..."
cargo build --release

echo ""
echo "🎯 Starting Plaur in bootstrap mode..."
echo "💡 This will initialize the LLM pipeline and show system status"
echo ""

# Run the main binary
cargo run --release

echo ""
echo "✅ Plaur bootstrap completed"