//! # Finmoney
//!
//! A precise, panic-free money library for Rust. It provides safe monetary arithmetic,
//! currency-aware values, configurable rounding strategies, and exchange-grade tick handling.
//! Designed for trading systems, bots, and financial apps where correctness and determinism matter.
//!
//! ## Features
//!
//! - **Precise arithmetic**: Built on `rust_decimal` for exact decimal calculations
//! - **Currency safety**: Prevents mixing different currencies in operations
//! - **Configurable rounding**: Multiple rounding strategies for different use cases
//! - **Tick handling**: Exchange-grade price/quantity rounding to valid tick sizes
//! - **Zero panics**: All operations return `Result` types for error handling
//! - **Serde support**: Optional serialization/deserialization (feature-gated)
//!
//! ## Quick Start
//!
//! ```rust
//! use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyRoundingStrategy};
//! use rust_decimal_macros::dec;
//!
//! // Create a currency
//! let usd = FinMoneyCurrency::new(1, "USD".to_string(), Some("US Dollar".to_string()), 2)?;
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
