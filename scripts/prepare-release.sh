#!/bin/bash
# Script to prepare a new release

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if version argument is provided
if [ $# -eq 0 ]; then
    print_error "Please provide a version number (e.g., 0.1.0)"
    exit 1
fi

VERSION=$1

print_status "Preparing release v$VERSION"

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    print_warning "You're not on the main branch. Current branch: $CURRENT_BRANCH"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    print_error "Working directory is not clean. Please commit or stash changes."
    exit 1
fi

# Update version in Cargo.toml
print_status "Updating version in Cargo.toml"
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Run tests
print_status "Running tests"
cargo test --all-features

# Run clippy
print_status "Running clippy"
cargo clippy --all-targets --all-features -- -D warnings

# Format code
print_status "Formatting code"
cargo fmt

# Build documentation
print_status "Building documentation"
cargo doc --no-deps --all-features

# Run examples
print_status "Testing examples"
cargo run --example basic_usage > /dev/null
cargo run --example trading_ticks > /dev/null

# Security audit
print_status "Running security audit"
if command -v cargo-audit &> /dev/null; then
    cargo audit
else
    print_warning "cargo-audit not found. Install with: cargo install cargo-audit"
fi

# Dry run publish
print_status "Running publish dry run"
cargo publish --dry-run

print_status "Release preparation complete!"
print_status "Next steps:"
echo "1. Review the changes"
echo "2. Update CHANGELOG.md"
echo "3. Commit changes: git add . && git commit -m 'Prepare release v$VERSION'"
echo "4. Create tag: git tag -a v$VERSION -m 'Release version $VERSION'"
echo "5. Push: git push origin main && git push origin v$VERSION"
echo "6. Publish: cargo publish"