# Makefile for finmoney

.PHONY: help test check fmt clippy doc examples bench clean install-tools audit publish-dry

# Default target
help:
	@echo "Available targets:"
	@echo "  test         - Run all tests"
	@echo "  check        - Check code without building"
	@echo "  fmt          - Format code"
	@echo "  clippy       - Run clippy linter"
	@echo "  doc          - Build documentation"
	@echo "  examples     - Run all examples"
	@echo "  bench        - Run benchmarks"
	@echo "  clean        - Clean build artifacts"
	@echo "  install-tools- Install development tools"
	@echo "  audit        - Run security audit"
	@echo "  publish-dry  - Dry run publish to crates.io"
	@echo "  all          - Run test, clippy, fmt, and doc"

# Run all tests
test:
	cargo test --all-features

# Check code without building
check:
	cargo check --all-targets --all-features

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Build documentation
doc:
	cargo doc --no-deps --all-features --open

# Run examples
examples:
	cargo run --example basic_usage
	cargo run --example trading_ticks

# Run benchmarks
bench:
	cargo bench

# Clean build artifacts
clean:
	cargo clean

# Install development tools
install-tools:
	cargo install cargo-audit
	cargo install cargo-outdated
	cargo install cargo-deny
	rustup component add clippy rustfmt

# Run security audit
audit:
	cargo audit

# Dry run publish
publish-dry:
	cargo publish --dry-run

# Run all quality checks
all: test clippy fmt doc
	@echo "All checks passed!"

# Development workflow
dev: fmt clippy test examples
	@echo "Development checks complete!"