# Publishing Guide

This document outlines the steps to publish finmoney to crates.io and maintain the GitHub repository.

## Prerequisites

1. **Crates.io Account**: Create an account at [crates.io](https://crates.io/)
2. **API Token**: Generate an API token from your crates.io account settings
3. **GitHub Repository**: Set up the repository on GitHub

## Pre-Publication Checklist

### 1. Code Quality
- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] Documentation builds: `cargo doc --no-deps --all-features`
- [ ] Examples work: `cargo run --example basic_usage`

### 2. Documentation
- [ ] README.md is up to date
- [ ] CHANGELOG.md includes new version
- [ ] All public APIs are documented
- [ ] Examples are working and relevant

### 3. Metadata
- [ ] Cargo.toml version is correct
- [ ] Cargo.toml metadata is complete (description, keywords, categories)
- [ ] License files are present
- [ ] Repository URL is correct

### 4. Security
- [ ] Run security audit: `cargo audit`
- [ ] No known vulnerabilities in dependencies
- [ ] Sensitive information is not included

## Publishing Steps

### 1. Prepare Release

```bash
# Update version in Cargo.toml
# Update CHANGELOG.md
# Commit changes
git add .
git commit -m "Prepare release v0.1.0"
git push origin main
```

### 2. Create Git Tag

```bash
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

### 3. Publish to Crates.io

```bash
# Login to crates.io (first time only)
cargo login

# Dry run to check everything
cargo publish --dry-run

# Publish to crates.io
cargo publish
```

### 4. Create GitHub Release

1. Go to your GitHub repository
2. Click "Releases" → "Create a new release"
3. Choose the tag you created (v0.1.0)
4. Add release title: "v0.1.0"
5. Copy relevant section from CHANGELOG.md to release notes
6. Publish release

## Post-Publication

### 1. Verify Publication
- [ ] Check crates.io page: https://crates.io/crates/finmoney
- [ ] Verify docs.rs builds: https://docs.rs/finmoney
- [ ] Test installation: `cargo install finmoney --dry-run`

### 2. Update Documentation
- [ ] Update README badges if needed
- [ ] Update any external documentation
- [ ] Announce on relevant forums/social media

## Version Management

### Semantic Versioning
- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.1.1): Bug fixes, backwards compatible

### Release Branches
- `main`: Stable releases
- `develop`: Development branch
- `feature/*`: Feature branches

## Maintenance

### Regular Tasks
- [ ] Update dependencies monthly
- [ ] Run security audits weekly
- [ ] Review and respond to issues
- [ ] Update documentation as needed

### Dependency Updates
```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Test after updates
cargo test --all-features
```

## Troubleshooting

### Common Issues

1. **Crate name already taken**
   - Choose a different name
   - Update Cargo.toml and documentation

2. **Documentation fails to build**
   - Check for broken doc links
   - Ensure all features compile
   - Test with `cargo doc --no-deps`

3. **Tests fail in CI**
   - Check platform-specific issues
   - Verify all feature combinations
   - Update CI configuration

4. **License issues**
   - Ensure license files are included
   - Check dependency licenses
   - Update deny.toml if needed

## Resources

- [Crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)