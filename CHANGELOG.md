# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-26

### Added
- Initial release of finmoney library
- `FinMoney` type for precise monetary calculations
- `finmoneyCurrency` type for currency representation
- Support for multiple rounding strategies
- Exchange-grade tick handling for trading applications
- Currency safety (prevents mixing different currencies)
- Comprehensive arithmetic operations
- Percentage calculations
- Mathematical operations (abs, sqrt, floor, ceil, etc.)
- Predefined common currencies (USD, EUR, BTC, ETH)
- Optional serde support for serialization
- Comprehensive test suite
- Benchmarks for performance testing
- Examples for basic usage and trading scenarios
- Full documentation with examples
- **Rust 2024 edition** support for modern language features

### Dependencies
- rust_decimal 1.39.0 (with maths feature)
- rust_decimal_macros 1.39.0
- tinystr 0.8.2 (with serde feature)
- serde 1.0.228 (optional, with derive feature)
- criterion 0.8.1 (dev dependency)

### Requirements
- Rust 1.90 or later (Rust 2024 edition)
