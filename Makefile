all: format lint test check build run

# Format code
format:
	cargo fmt

# Run clippy for linting
lint:
	cargo clippy

# Run tests
test:
	cargo test

# Check if code compiles
check:
	cargo check

# Build release version
build:
	cargo build --release

# Run the program
run:
	cargo run