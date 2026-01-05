.PHONY: build test clean docker-build docker-run deploy-lambda setup-dev

# Default target
all: build

# Build all services
build:
	@echo "Building Plaur services..."
	cargo build --release

# Build for Lambda
build-lambda:
	@echo "Building for AWS Lambda..."
	./build.sh

# Run tests
test:
	@echo "Running tests..."
	cargo test --all

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf deployment/
	rm -f bootstrap plaur-deployment.zip

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt --all

# Lint code
lint:
	@echo "Linting code..."
	cargo clippy --all-targets --all-features -- -D warnings

# Check code
check:
	@echo "Checking code..."
	cargo check --all

# Setup development environment
setup-dev:
	@echo "Setting up development environment..."
	rustup target add x86_64-unknown-linux-musl
	cargo install sqlx-cli
	docker-compose up -d postgres redis qdrant

# Run database migrations
migrate:
	@echo "Running database migrations..."
	sqlx migrate run

# Start local services
dev:
	@echo "Starting development services..."
	docker-compose up -d

# Stop local services
dev-stop:
	@echo "Stopping development services..."
	docker-compose down

# Build Docker images
docker-build:
	@echo "Building Docker images..."
	docker build --platform linux/x86_64 -t plaur:latest .
	docker build --platform linux/x86_64 -t plaur-lambda:latest -f Dockerfile.lambda .

# Run with Docker Compose
docker-run:
	@echo "Running with Docker Compose..."
	docker-compose up --build

# Deploy to AWS Lambda
deploy-lambda: build-lambda
	@echo "Deploying to AWS Lambda..."
	docker build --platform linux/x86_64 -t plaur-auth:latest -f Dockerfile.lambda .
	docker create --name temp-plaur plaur-auth:latest
	docker cp temp-plaur:/var/runtime/bootstrap ./bootstrap
	docker rm temp-plaur
	zip -j plaur-deployment.zip bootstrap
	@echo "Deployment package created: plaur-deployment.zip"

# Generate documentation
docs:
	@echo "Generating documentation..."
	cargo doc --all --no-deps

# Security audit
audit:
	@echo "Running security audit..."
	cargo audit

# Update dependencies
update:
	@echo "Updating dependencies..."
	cargo update

# Install development tools
install-tools:
	@echo "Installing development tools..."
	cargo install cargo-watch cargo-audit sqlx-cli
	rustup component add clippy rustfmt