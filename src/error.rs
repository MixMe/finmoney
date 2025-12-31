//! Error types for the finmoney library.

use std::fmt;

/// Errors that can occur during money operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyError {
    /// Attempted to perform an operation between different currencies.
    CurrencyMismatch {
        /// The expected currency code.
        expected: String,
        /// The actual currency code that was provided.
        actual: String,
    },
    /// Attempted division by zero.
    DivisionByZero,
    /// Invalid precision value (must be <= 28 for Decimal compatibility).
    InvalidPrecision(u32),
    /// Invalid tick size (must be positive).
    InvalidTick,
    /// Currency code is invalid or too long.
    InvalidCurrencyCode(String),
    /// Currency name is invalid or too long.
    InvalidCurrencyName(String),
    /// Arithmetic overflow occurred during calculation.
    ArithmeticOverflow,
    /// Invalid amount (e.g., NaN or infinite values).
    InvalidAmount(String),
}

impl fmt::Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoneyError::CurrencyMismatch { expected, actual } => write!(
                f,
                "Currency mismatch: expected {}, got {}",
                expected, actual
            ),
            MoneyError::DivisionByZero => write!(f, "Division by zero"),
            MoneyError::InvalidPrecision(p) => {
                write!(f, "Invalid precision: {} (must be <= 28)", p)
            }
            MoneyError::InvalidTick => write!(f, "Invalid tick size (must be positive)"),
            MoneyError::InvalidCurrencyCode(code) => write!(f, "Invalid currency code: {}", code),
            MoneyError::InvalidCurrencyName(name) => write!(f, "Invalid currency name: {}", name),
            MoneyError::ArithmeticOverflow => write!(f, "Arithmetic overflow occurred"),
            MoneyError::InvalidAmount(msg) => write!(f, "Invalid amount: {}", msg),
        }
    }
}

impl std::error::Error for MoneyError {}

/// Result type alias for operations that can fail with `MoneyError`.
pub type Result<T> = std::result::Result<T, MoneyError>;
