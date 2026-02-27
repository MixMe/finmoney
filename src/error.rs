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
            FinMoneyError::InvalidCurrencyCode(code) => {
                write!(f, "Invalid currency code: {}", code)
            }
            FinMoneyError::InvalidCurrencyName(name) => {
                write!(f, "Invalid currency name: {}", name)
            }
            FinMoneyError::ArithmeticOverflow => write!(f, "Arithmetic overflow occurred"),
            FinMoneyError::InvalidAmount(msg) => write!(f, "Invalid amount: {}", msg),
        }
    }
}

impl std::error::Error for FinMoneyError {}

impl FinMoneyError {
    /// Returns `true` if this error is a currency mismatch.
    ///
    /// # Examples
    ///
    /// ```
    /// use finmoney::FinMoneyError;
    ///
    /// let err = FinMoneyError::CurrencyMismatch {
    ///     expected: "USD".into(),
    ///     actual: "EUR".into(),
    /// };
    /// assert!(err.is_currency_mismatch());
    /// assert!(!FinMoneyError::DivisionByZero.is_currency_mismatch());
    /// ```
    pub fn is_currency_mismatch(&self) -> bool {
        matches!(self, FinMoneyError::CurrencyMismatch { .. })
    }

    /// Returns `true` if this error is a division by zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use finmoney::FinMoneyError;
    ///
    /// assert!(FinMoneyError::DivisionByZero.is_division_by_zero());
    /// assert!(!FinMoneyError::ArithmeticOverflow.is_division_by_zero());
    /// ```
    pub fn is_division_by_zero(&self) -> bool {
        matches!(self, FinMoneyError::DivisionByZero)
    }

    /// Returns `true` if this error is an arithmetic overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use finmoney::FinMoneyError;
    ///
    /// assert!(FinMoneyError::ArithmeticOverflow.is_overflow());
    /// assert!(!FinMoneyError::DivisionByZero.is_overflow());
    /// ```
    pub fn is_overflow(&self) -> bool {
        matches!(self, FinMoneyError::ArithmeticOverflow)
    }
}

/// Result type alias for operations that can fail with `FinMoneyError`.
pub type Result<T> = std::result::Result<T, FinMoneyError>;
