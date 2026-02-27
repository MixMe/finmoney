//! # Finmoney
//!
//! A precise, panic-free money library for Rust. It provides safe monetary arithmetic,
//! currency-aware values, configurable rounding strategies, and exchange-grade tick handling.
//! Designed for trading systems, bots, and financial apps where correctness and determinism matter.
//!
//! ## Architecture
//!
//! The crate is organized into four modules, each with a clear responsibility:
//!
//! - [`FinMoney`] — the core type representing a monetary value bound to a currency.
//!   Supports checked arithmetic, allocation, currency conversion, formatting, and
//!   tick-based rounding.
//! - [`FinMoneyCurrency`] — represents a currency with a code, optional name, and
//!   decimal precision. Ships with 11 predefined currencies (USD, EUR, BTC, ETH,
//!   GBP, JPY, CHF, CNY, RUB, USDT, SOL).
//! - [`FinMoneyRoundingStrategy`] — an enum of rounding strategies used by arithmetic
//!   and conversion operations.
//! - [`FinMoneyError`] — a comprehensive error enum covering currency mismatches,
//!   overflow, division by zero, and invalid inputs. Provides convenience predicates
//!   like [`FinMoneyError::is_overflow()`].
//!
//! ## Features
//!
//! - **Precise arithmetic** — built on `rust_decimal` for exact decimal calculations.
//!   All operations use checked arithmetic and return `Result` on overflow.
//! - **Currency safety** — prevents mixing different currencies in operations.
//! - **Configurable rounding** — multiple [`FinMoneyRoundingStrategy`] variants for
//!   different use cases.
//! - **Tick handling** — exchange-grade price/quantity rounding to valid tick sizes.
//! - **Allocation** — split a monetary amount by weights without losing a single cent
//!   via [`FinMoney::allocate()`].
//! - **Currency conversion** — convert between currencies at a given rate with
//!   [`FinMoney::convert_to()`], or compute the implied rate with
//!   [`FinMoney::exchange_rate_to()`].
//! - **Iterator support** — [`FinMoney`] implements [`Sum`](std::iter::Sum) and
//!   provides [`FinMoney::try_sum()`] as a safe, non-panicking alternative.
//! - **Flexible formatting** — [`FinMoney::format_with_separator()`] for
//!   locale-aware thousand separators and [`FinMoney::format_padded()`] for
//!   zero-padded decimal output.
//! - **Serde support** — optional serialization/deserialization (feature `serde`).
//!   Amounts are serialized as strings to preserve precision.
//!
//! ## Quick Start
//!
//! ```rust
//! use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyRoundingStrategy};
//! use rust_decimal_macros::dec;
//!
//! // Create a currency
//! let usd = FinMoneyCurrency::new(1, "USD", Some("US Dollar"), 2)?;
//!
//! // Create money values
//! let price = FinMoney::new(dec!(10.50), usd);
//! let quantity = FinMoney::new(dec!(3), usd);
//!
//! // Perform arithmetic
//! let total = (price + quantity)?;
//! println!("{}", total); // 13.50 USD
//!
//! // Round to tick size
//! let rounded = price.to_tick_nearest(dec!(0.25))?;
//! # Ok::<(), finmoney::FinMoneyError>(())
//! ```
//!
//! ## Allocation
//!
//! Split a sum into weighted parts with zero remainder loss:
//!
//! ```rust
//! use finmoney::{FinMoney, FinMoneyCurrency, dec};
//!
//! let usd = FinMoneyCurrency::USD;
//! let total = FinMoney::new(dec!(100), usd);
//! let parts = total.allocate(&[dec!(1), dec!(1), dec!(1)])?;
//! // parts: [33.34, 33.33, 33.33] — sum is exactly 100.00
//! # Ok::<(), finmoney::FinMoneyError>(())
//! ```
//!
//! ## Currency Conversion
//!
//! ```rust
//! use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyRoundingStrategy, dec};
//!
//! let usd_amount = FinMoney::new(dec!(100), FinMoneyCurrency::USD);
//! let eur = FinMoneyCurrency::EUR;
//! let eur_amount = usd_amount.convert_to(eur, dec!(0.92), FinMoneyRoundingStrategy::MidpointNearestEven)?;
//! // 92.00 EUR
//! # Ok::<(), finmoney::FinMoneyError>(())
//! ```
//!
//! ## Summing Iterators
//!
//! ```rust
//! use finmoney::{FinMoney, FinMoneyCurrency, dec};
//!
//! let usd = FinMoneyCurrency::USD;
//! let values = vec![
//!     FinMoney::new(dec!(10), usd),
//!     FinMoney::new(dec!(20), usd),
//!     FinMoney::new(dec!(30), usd),
//! ];
//!
//! // Safe version (returns Result)
//! let total = FinMoney::try_sum(values.into_iter())?;
//! // 60.00 USD
//! # Ok::<(), finmoney::FinMoneyError>(())
//! ```
//!
//! ## Formatting
//!
//! ```rust
//! use finmoney::{FinMoney, FinMoneyCurrency, dec};
//!
//! let m = FinMoney::new(dec!(1234567.89), FinMoneyCurrency::USD);
//! assert_eq!(m.format_with_separator(',', '.'), "1,234,567.89 USD");
//! assert_eq!(m.format_padded(4), "1234567.8900 USD");
//! ```

pub mod currency;
pub mod error;
pub mod money;
pub mod rounding;

pub use currency::FinMoneyCurrency;
pub use error::FinMoneyError;
pub use money::FinMoney;
pub use rounding::FinMoneyRoundingStrategy;

// Re-export commonly used types from dependencies
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
