# Contributing to Finmoney

Thank you for your interest in contributing to Finmoney! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/finmoney.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run benchmarks: `cargo bench`
7. Commit your changes: `git commit -am 'Add some feature'`
8. Push to the branch: `git push origin feature/your-feature-name`
9. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.90 or later (Rust 2024 edition)
- Cargo

### Building

```bash
cargo build
```

### Testing

Run all tests:
```bash
cargo test
```

Run specific test modules:
```bash
cargo test money_tests
cargo test currency_tests
cargo test tick_tests
```

### Benchmarking

```bash
cargo bench
```

### Examples

Run the examples to see the library in action:
```bash
cargo run --example basic_usage
cargo run --example trading_ticks
```

## Code Style

- Follow standard Rust formatting: `cargo fmt`
- Run clippy for linting: `cargo clippy`
- Ensure all tests pass
- Add tests for new functionality
- Update documentation for public APIs

## Documentation

- All public APIs must be documented
- Include examples in documentation where helpful
- Run `cargo doc --open` to view generated documentation

## Pull Request Guidelines

1. **Description**: Provide a clear description of what your PR does
2. **Tests**: Include tests for new functionality
3. **Documentation**: Update documentation for any API changes
4. **Backwards Compatibility**: Avoid breaking changes when possible
5. **Performance**: Consider performance implications of changes

## Areas for Contribution

### High Priority
- Additional currency definitions
- Performance optimizations
- More comprehensive error handling
- Additional rounding strategies

### Medium Priority
- Integration with popular financial libraries
- Additional mathematical operations
- Improved serialization formats
- More examples and tutorials

### Low Priority
- Additional benchmarks
- Code organization improvements
- Documentation enhancements

## Reporting Issues

When reporting issues, please include:

1. **Version**: Which version of finmoney you're using
2. **Environment**: Rust version, operating system
3. **Description**: Clear description of the issue
4. **Reproduction**: Minimal code example that reproduces the issue
5. **Expected vs Actual**: What you expected vs what actually happened

## Feature Requests

For feature requests, please:

1. Check if the feature already exists or is planned
2. Provide a clear use case for the feature
3. Consider backwards compatibility
4. Be willing to help implement the feature

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Questions?

Feel free to open an issue for questions about contributing or using the library.