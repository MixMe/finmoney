//! # Finmoney
//!
//! A precise money library for Rust. Provides currency-aware arithmetic, exchange-grade
//! tick handling, configurable rounding, and fair allocation. Built on `rust_decimal`
//! for exact decimal calculations. Designed for trading systems, bots, and financial
//! apps where correctness and determinism matter.
//!
//! ## Architecture
//!
//! The crate is organized into four modules, each with a clear responsibility:
//!
//! - [`FinMoney`] — the core type representing a monetary value bound to a currency.
//!   Supports arithmetic, allocation, currency conversion, formatting, and
//!   tick-based rounding.
//! - [`FinMoneyCurrency`] — represents a currency with a code, optional name, and
//!   decimal precision. No predefined currencies — create your own via
//!   `FinMoneyCurrency::new()`.
//! - [`FinMoneyRoundingStrategy`] — an enum of rounding strategies used by arithmetic
//!   and conversion operations.
//! - [`FinMoneyError`] — error enum covering currency mismatches, division by zero,
//!   and invalid inputs.
//!
//! ## Design Principles
//!
//! - **Ergonomic by default.** Comparisons (`<`, `>`, `<=`, `>=`), scalar arithmetic
//!   (`+ Decimal`, `* Decimal`), and in-place operations (`+=`, `-=`, `*=`) return
//!   direct values.
//! - **Result where it matters.** Only operations with real runtime failure modes
//!   (currency mismatch between two `FinMoney` values, division by zero) return `Result`.
//! - **Checked variants available.** Every panicking method has a `Result`-returning
//!   counterpart (`unchecked_plus` ↔ `plus_money`, `*` ↔ `multiplied_by_decimal`).
//! - **Precise.** `rust_decimal` 128-bit decimals — no floating-point surprises.
//! - **Currency-agnostic.** No predefined currencies — define any currency with any
//!   precision for any domain (fiat, crypto, game tokens, loyalty points).
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
//! let _rounded = price.to_tick_nearest(dec!(0.25))?;
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
//! let usd = FinMoneyCurrency::new(1, "USD", None::<&str>, 2)?;
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
//! let usd_cur = FinMoneyCurrency::new(1, "USD", None::<&str>, 2)?;
//! let eur_cur = FinMoneyCurrency::new(3, "EUR", None::<&str>, 2)?;
//! let usd_amount = FinMoney::new(dec!(100), usd_cur);
//! let eur_amount = usd_amount.convert_to(eur_cur, dec!(0.92), FinMoneyRoundingStrategy::MidpointNearestEven)?;
//! // 92.00 EUR
//! # Ok::<(), finmoney::FinMoneyError>(())
//! ```
//!
//! ## Summing Iterators
//!
//! ```rust
//! use finmoney::{FinMoney, FinMoneyCurrency, dec};
//!
//! let usd = FinMoneyCurrency::new(1, "USD", None::<&str>, 2)?;
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
//! let usd = FinMoneyCurrency::new(1, "USD", None::<&str>, 2).unwrap();
//! let m = FinMoney::new(dec!(1234567.89), usd);
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
