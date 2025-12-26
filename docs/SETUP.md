# Repository Setup Guide

This guide will help you set up the finmoney repository on GitHub and prepare it for publication to crates.io.

## GitHub Repository Setup

### 1. Create Repository

1. Go to [GitHub](https://github.com) and create a new repository
2. Repository name: `finmoney`
3. Description: "A precise, panic-free money library for Rust"
4. Make it public
5. Don't initialize with README (we already have one)

### 2. Push Local Repository

```bash
# Initialize git (if not already done)
git init

# Add all files
git add .

# Initial commit
git commit -m "Initial commit: finmoney library v0.1.0"

# Add remote origin (replace with your username)
git remote add origin https://github.com/yourusername/finmoney.git

# Push to GitHub
git branch -M main
git push -u origin main
```

### 3. Configure Repository Settings

#### Branch Protection
1. Go to Settings → Branches
2. Add rule for `main` branch:
   - Require pull request reviews before merging
   - Require status checks to pass before merging
   - Require branches to be up to date before merging
   - Include administrators

#### Repository Settings
1. Go to Settings → General
2. Features:
   - ✅ Issues
   - ✅ Projects
   - ✅ Wiki
   - ✅ Discussions (optional)
3. Pull Requests:
   - ✅ Allow squash merging
   - ✅ Allow auto-merge
   - ✅ Automatically delete head branches

#### Secrets (for CI)
1. Go to Settings → Secrets and variables → Actions
2. Add repository secrets if needed (e.g., for deployment)

## Crates.io Setup

### 1. Create Account
1. Go to [crates.io](https://crates.io/)
2. Sign in with GitHub
3. Verify your email

### 2. Generate API Token
1. Go to Account Settings
2. Click "New Token"
3. Name: "finmoney-publishing"
4. Copy the token (you'll need it for publishing)

### 3. Configure Cargo
```bash
# Login to crates.io (paste your token when prompted)
cargo login
```

## Pre-Publication Checklist

### 1. Update Personal Information

Replace placeholder information in these files:
- `Cargo.toml`: Update author name and email
- `Cargo.toml`: Update repository URL to your GitHub repo
- `README.md`: Update any references to repository URL
- `CONTRIBUTING.md`: Update repository references

### 2. Verify Metadata

Check `Cargo.toml` for:
- Correct version number
- Accurate description
- Proper keywords and categories
- Valid license
- Correct repository URL

### 3. Test Everything

```bash
# Run all tests
cargo test --all-features

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build docs
cargo doc --no-deps --all-features

# Test examples
cargo run --example basic_usage
cargo run --example trading_ticks

# Security audit (install if needed: cargo install cargo-audit)
cargo audit

# Dry run publish
cargo publish --dry-run
```

## Publishing Process

### 1. Prepare Release

Use the provided script:
```bash
# On Unix/Linux/macOS
./scripts/prepare-release.sh 0.1.0

# On Windows
scripts\prepare-release.bat 0.1.0
```

Or manually:
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run all tests and checks
4. Commit changes

### 2. Create Release

```bash
# Commit release preparation
git add .
git commit -m "Prepare release v0.1.0"

# Create and push tag
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin main
git push origin v0.1.0
```

### 3. Publish to Crates.io

```bash
# Final check
cargo publish --dry-run

# Publish
cargo publish
```

### 4. Create GitHub Release

1. Go to your repository on GitHub
2. Click "Releases" → "Create a new release"
3. Choose tag: `v0.1.0`
4. Release title: `v0.1.0`
5. Description: Copy from CHANGELOG.md
6. Publish release

## Post-Publication

### 1. Verify Publication
- Check [crates.io/crates/finmoney](https://crates.io/crates/finmoney)
- Verify [docs.rs/finmoney](https://docs.rs/finmoney) builds correctly
- Test installation: `cargo install finmoney --dry-run`

### 2. Update README Badges

Add these badges to your README.md:

```markdown
[![Crates.io](https://img.shields.io/crates/v/finmoney.svg)](https://crates.io/crates/finmoney)
[![Documentation](https://docs.rs/finmoney/badge.svg)](https://docs.rs/finmoney)
[![Build Status](https://github.com/yourusername/finmoney/workflows/CI/badge.svg)](https://github.com/yourusername/finmoney/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
```

### 3. Announce

Consider announcing your crate on:
- [r/rust](https://reddit.com/r/rust)
- [Rust Users Forum](https://users.rust-lang.org/)
- [This Week in Rust](https://this-week-in-rust.org/) (submit to their newsletter)
- Twitter/X with #rustlang hashtag

## Maintenance

### Regular Tasks
- Monitor issues and pull requests
- Update dependencies monthly
- Run security audits weekly
- Keep documentation up to date

### Dependency Updates
```bash
# Check for updates
cargo outdated

# Update Cargo.lock
cargo update

# Test after updates
cargo test --all-features
```

## Troubleshooting

### Common Issues

1. **Crate name taken**: Choose a different name
2. **CI failures**: Check platform compatibility
3. **Documentation issues**: Ensure all doc links work
4. **License problems**: Verify all dependencies are compatible

### Getting Help

- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [GitHub Issues](https://github.com/yourusername/finmoney/issues)

## Resources

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Crates.io Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Keep a Changelog](https://keepachangelog.com/)