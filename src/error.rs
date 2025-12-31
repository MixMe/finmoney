//! Error types for the finmoney library.

use std::fmt;

/// Errors that can occur during money operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinMoneyError {
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

impl fmt::Display for FinMoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FinMoneyError::CurrencyMismatch { expected, actual } => write!(
                f,
                "Currency mismatch: expected {}, got {}",
                expected, actual
            ),
            FinMoneyError::DivisionByZero => write!(f, "Division by zero"),
            FinMoneyError::InvalidPrecision(p) => {
                write!(f, "Invalid precision: {} (must be <= 28)", p)
            }
            FinMoneyError::InvalidTick => write!(f, "Invalid tick size (must be positive)"),
            FinMoneyError::InvalidCurrencyCode(code) => write!(f, "Invalid currency code: {}", code),
            FinMoneyError::InvalidCurrencyName(name) => write!(f, "Invalid currency name: {}", name),
            FinMoneyError::ArithmeticOverflow => write!(f, "Arithmetic overflow occurred"),
            FinMoneyError::InvalidAmount(msg) => write!(f, "Invalid amount: {}", msg),
        }
    }
}

impl std::error::Error for FinMoneyError {}

/// Result type alias for operations that can fail with `FinMoneyError`.
pub type Result<T> = std::result::Result<T, FinMoneyError>;
