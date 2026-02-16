# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [2.0.1] - 2025-02-16

### Fixed
- Updated README examples to reflect 2.0.0 API changes (removed `.to_string()` boilerplate, added `Neg` operator and reverse `Mul` examples)
- Updated version references in README

## [2.0.0] - 2025-02-16

### Breaking Changes
- Removed `AddAssign` and `SubAssign` operator implementations (they panicked on currency mismatch, violating the panic-free guarantee)
- Removed `compare_to()` method (use `compare()` instead)
- Removed `is_amount_and_currency_equal_to()` method (use `is_equal_to()` instead)
- `sqrt()` now returns `Result<FinMoney, FinMoneyError>` instead of panicking on negative input
- `tick_power10_dp()` is now private (was an internal helper exposed unnecessarily)
- `FinMoneyCurrency::new()` name parameter changed from `Option<String>` to `Option<impl Into<String>>`

### Added
- `Eq` derive on `FinMoney`
- `Neg` operator implementation (`-money` instead of `money.negated()`)
- `Mul<FinMoney> for Decimal` (reverse multiplication: `dec!(2) * money`)
- `#[inline]` on all hot-path methods across `FinMoney` and `FinMoneyCurrency`

### Changed
- `FinMoneyCurrency::new()` now accepts `impl Into<String>` for both `code` and `name` parameters, eliminating `.to_string()` boilerplate at call sites
- Updated criterion dev-dependency from 0.8.1 to 0.8.2

## [1.0.5] - 2024-12-31

### Changed
- Rust Decimal updated 1.39.0 -> 1.40.0


## [1.0.4] - 2024-12-31

### Changed
- **BREAKING**: Renamed `MoneyError` to `FinMoneyError` for consistency with other types
- **BREAKING**: Renamed `MoneyRoundingStrategy` to `FinMoneyRoundingStrategy` for consistency with other types
- All function signatures and documentation updated to use the new type names


## [1.0.3] - 2024-12-31

### Added
- `FinMoneyCurrency::new_from_tiny()` method for creating currencies with pre-calculated `TinyAsciiStr` values
  - More efficient than `new()` when working with pre-validated currency data
  - Avoids string parsing and sanitization overhead
  - Useful for performance-critical applications


## [1.0.2] - 2024-12-30

### Added
- Performance optimizations for currency creation
- Enhanced documentation with more examples


## [1.0.0] - 2024-12-26

### Added
- Initial release of finmoney library
- `FinMoney` type for precise monetary calculations
- `FinMoneyCurrency` type for currency representation
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
