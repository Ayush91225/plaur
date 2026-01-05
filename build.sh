#!/bin/bash

set -e

echo "Building Plaur for AWS Lambda deployment..."

# Set environment variables
export CARGO_TARGET_DIR=target
export RUSTFLAGS="-C target-cpu=native"

# Clean previous builds
cargo clean

# Build for Lambda (x86_64-unknown-linux-musl)
echo "Building for Lambda runtime..."
rustup target add x86_64-unknown-linux-musl

# Build all services
echo "Building auth service..."
cargo build --release --bin plaur-auth --target x86_64-unknown-linux-musl

echo "Building API gateway..."
cargo build --release --bin plaur-api --target x86_64-unknown-linux-musl

echo "Building ingestion service..."
cargo build --release --bin plaur-ingestion --target x86_64-unknown-linux-musl

# Build Lambda functions
echo "Building Lambda functions..."
cargo build --release --bin shopify-webhook --target x86_64-unknown-linux-musl

# Create deployment directory
mkdir -p deployment

# Copy binaries and rename for Lambda
cp target/x86_64-unknown-linux-musl/release/plaur-auth deployment/bootstrap-auth
cp target/x86_64-unknown-linux-musl/release/plaur-api deployment/bootstrap-api
cp target/x86_64-unknown-linux-musl/release/plaur-ingestion deployment/bootstrap-ingestion
cp target/x86_64-unknown-linux-musl/release/shopify-webhook deployment/bootstrap-webhook

# Make executable
chmod +x deployment/bootstrap-*

echo "Build completed successfully!"
echo "Binaries available in deployment/ directory"

# Docker build command for Lambda
echo ""
echo "To build Docker image for Lambda deployment:"
echo "docker build --platform linux/x86_64 -t plaur-auth:latest -f Dockerfile.lambda ."
echo "docker create --name temp-plaur plaur-auth:latest"
echo "docker cp temp-plaur:/var/runtime/bootstrap ./bootstrap"
echo "docker rm temp-plaur"
echo "zip -j plaur-deployment.zip bootstrap"