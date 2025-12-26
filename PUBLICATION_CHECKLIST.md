# Publication Checklist for Finmoney

This checklist ensures your finmoney library is ready for GitHub and crates.io publication.

## ✅ Pre-Publication Setup

### 1. Personal Information Updates
- [ ] Update `Cargo.toml` author name and email
- [ ] Update `Cargo.toml` repository URL (replace `yourusername` with your GitHub username)
- [ ] Update `README.md` repository references
- [ ] Update `CONTRIBUTING.md` repository references
- [ ] Update `docs/SETUP.md` with your information

### 2. GitHub Repository Setup
- [ ] Create repository on GitHub: `https://github.com/yourusername/finmoney`
- [ ] Initialize local git repository: `git init`
- [ ] Add remote: `git remote add origin https://github.com/yourusername/finmoney.git`
- [ ] Push initial commit: `git add . && git commit -m "Initial commit" && git push -u origin main`

### 3. Crates.io Account Setup
- [ ] Create account at [crates.io](https://crates.io/)
- [ ] Generate API token
- [ ] Login with cargo: `cargo login`

## ✅ Quality Assurance

### Code Quality
- [ ] All tests pass: `cargo test --all-features`
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Code is formatted: `cargo fmt`
- [ ] Documentation builds: `cargo doc --no-deps --all-features`

### Examples and Documentation
- [ ] Basic usage example works: `cargo run --example basic_usage`
- [ ] Trading ticks example works: `cargo run --example trading_ticks`
- [ ] All public APIs are documented
- [ ] README.md is comprehensive and accurate

### Security and Dependencies
- [ ] Security audit passes: `cargo audit` (install with `cargo install cargo-audit`)
- [ ] No known vulnerabilities
- [ ] Dependencies are up to date (rust_decimal 1.39.0, tinystr 0.8.2, serde 1.0.228, criterion 0.8.1)
- [ ] Rust 1.90+ installed (required for Rust 2024 edition)

### Publication Readiness
- [ ] Dry run succeeds: `cargo publish --dry-run`
- [ ] Version number is correct in `Cargo.toml`
- [ ] CHANGELOG.md is updated
- [ ] All required metadata is in `Cargo.toml`

## ✅ Publication Process

### 1. Final Preparation
```bash
# Run validation script (Unix/Linux/macOS)
./scripts/validate-release.sh

# Or on Windows
scripts\validate-release.bat

# Or use the preparation script
./scripts/prepare-release.sh 0.1.0  # Unix/Linux/macOS
scripts\prepare-release.bat 0.1.0  # Windows
```

### 2. Create Release
```bash
# Commit final changes
git add .
git commit -m "Prepare release v0.1.0"

# Create and push tag
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin main
git push origin v0.1.0
```

### 3. Publish to Crates.io
```bash
# Final dry run
cargo publish --dry-run

# Publish
cargo publish
```

### 4. Create GitHub Release
1. Go to your repository on GitHub
2. Click "Releases" → "Create a new release"
3. Choose tag: `v0.1.0`
4. Release title: `v0.1.0`
5. Description: Copy relevant section from CHANGELOG.md
6. Publish release

## ✅ Post-Publication

### Verification
- [ ] Check crates.io page: `https://crates.io/crates/finmoney`
- [ ] Verify docs.rs builds: `https://docs.rs/finmoney`
- [ ] Test installation: `cargo install finmoney --dry-run`

### Documentation Updates
- [ ] Update README badges with actual URLs
- [ ] Verify all links work
- [ ] Update any external documentation

### Announcement (Optional)
- [ ] Post on [r/rust](https://reddit.com/r/rust)
- [ ] Share on [Rust Users Forum](https://users.rust-lang.org/)
- [ ] Tweet with #rustlang hashtag
- [ ] Submit to [This Week in Rust](https://this-week-in-rust.org/)

## 📁 Project Structure Overview

Your finmoney project includes:

```
finmoney/
├── .github/
│   ├── workflows/          # CI/CD workflows
│   ├── ISSUE_TEMPLATE/     # Issue templates
│   └── pull_request_template.md
├── benches/                # Performance benchmarks
├── docs/                   # Additional documentation
├── examples/               # Usage examples
├── scripts/                # Utility scripts
├── src/                    # Source code
├── tests/                  # Integration tests
├── Cargo.toml             # Project configuration
├── README.md              # Main documentation
├── CHANGELOG.md           # Version history
├── CONTRIBUTING.md        # Contribution guidelines
├── LICENSE-MIT            # MIT license
├── LICENSE-APACHE         # Apache 2.0 license
├── clippy.toml           # Clippy configuration
├── rustfmt.toml          # Rustfmt configuration
├── deny.toml             # Cargo-deny configuration
└── Makefile              # Common tasks
```

## 🛠 Available Tools and Scripts

### Make Commands (if make is available)
```bash
make test          # Run all tests
make clippy        # Run clippy linter
make fmt           # Format code
make doc           # Build documentation
make examples      # Run examples
make all           # Run all quality checks
```

### Scripts
- `scripts/prepare-release.sh` / `.bat` - Prepare a new release
- `scripts/validate-release.sh` / `.bat` - Validate project is ready

### Cargo Commands
```bash
cargo test --all-features              # Run all tests
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo fmt                              # Format
cargo doc --no-deps --all-features     # Build docs
cargo audit                            # Security audit
cargo publish --dry-run               # Test publish
```

## 🔧 Maintenance

### Regular Tasks
- Update dependencies monthly: `cargo update`
- Run security audits weekly: `cargo audit`
- Review and respond to issues and PRs
- Keep documentation up to date

### Version Updates
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run quality checks
4. Create git tag
5. Publish to crates.io
6. Create GitHub release

## 📚 Resources

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)

---

**Ready to publish?** Follow this checklist step by step, and your finmoney library will be ready for the Rust community! 🦀