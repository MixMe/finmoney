//! Currency representation and management.

use crate::error::{MoneyError, Result};
use tinystr::TinyAsciiStr;

/// Represents a currency with an identifier, optional name, code, and precision.
///
/// The currency defines how monetary values should be formatted and rounded.
/// Each currency has a unique ID, a code (like "USD", "EUR"), an optional human-readable name,
/// and a precision that determines how many decimal places are significant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FinMoneyCurrency {
    id: i32,
    name: Option<TinyAsciiStr<52>>,
    code: TinyAsciiStr<16>,
    precision: u8,
}

impl Default for FinMoneyCurrency {
    /// Creates a default currency with undefined properties.
    ///
    /// This is primarily used as a fallback and should not be used in production code.
    fn default() -> Self {
        Self {
            id: 0,
            name: None,
            code: "UNDEFINED".parse().unwrap(),
            precision: 8,
        }
    }
}

impl FinMoneyCurrency {
    /// Creates a new currency with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the currency
    /// * `code` - Currency code (e.g., "USD", "EUR", "BTC")
    /// * `name` - Optional human-readable name (e.g., "US Dollar")
    /// * `precision` - Number of decimal places (must be <= 28)
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::InvalidPrecision` if precision > 28.
    /// Returns `MoneyError::InvalidCurrencyCode` if the code is invalid.
    /// Returns `MoneyError::InvalidCurrencyName` if the name is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::FinMoneyCurrency;
    ///
    /// let usd = FinMoneyCurrency::new(1, "USD".to_string(), Some("US Dollar".to_string()), 2)?;
    /// let btc = FinMoneyCurrency::new(2, "BTC".to_string(), Some("Bitcoin".to_string()), 8)?;
    /// # Ok::<(), finmoney::MoneyError>(())
    /// ```
    pub fn new(
        id: i32,
        code: String,
        name: Option<String>,
        precision: u8,
    ) -> Result<FinMoneyCurrency> {
        if precision > 28 {
            return Err(MoneyError::InvalidPrecision(precision as u32));
        }

        let parsed_name = match name {
            Some(n) => match Self::sanitize_and_parse_name(&n) {
                Ok(ascii_name) => Some(ascii_name),
                Err(_) => return Err(MoneyError::InvalidCurrencyName(n)),
            },
            None => None,
        };

        let parsed_code = Self::sanitize_and_parse_code(&code)
            .map_err(|_| MoneyError::InvalidCurrencyCode(code))?;

        Ok(Self {
            id,
            name: parsed_name,
            code: parsed_code,
            precision,
        })
    }

    /// Creates a new currency with basic parameters, using a fallback for invalid inputs.
    ///
    /// This method is more lenient than `new()` and will sanitize invalid characters
    /// rather than returning an error. Use this when you need guaranteed success
    /// but be aware that the resulting currency might have modified codes/names.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the currency
    /// * `code` - Currency code (will be sanitized if invalid)
    /// * `name` - Optional human-readable name (will be sanitized if invalid)
    /// * `precision` - Number of decimal places (will be clamped to 28 if too large)
    pub fn new_sanitized(
        id: i32,
        code: String,
        name: Option<String>,
        precision: u8,
    ) -> FinMoneyCurrency {
        let clamped_precision = precision.min(28);

        let sanitized_name = name.and_then(|n| Self::sanitize_and_parse_name(&n).ok());
        let sanitized_code =
            Self::sanitize_and_parse_code(&code).unwrap_or_else(|_| "INVALID".parse().unwrap());

        Self {
            id,
            name: sanitized_name,
            code: sanitized_code,
            precision: clamped_precision,
        }
    }

    /// Returns the unique identifier of this currency.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Returns the human-readable name of this currency, if available.
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the currency code (e.g., "USD", "EUR").
    pub fn get_code(&self) -> &str {
        &self.code
    }

    /// Returns the precision (number of decimal places) for this currency.
    pub fn get_precision(&self) -> u8 {
        self.precision
    }

    /// Creates a new currency with the same properties but different precision.
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::InvalidPrecision` if precision > 28.
    pub fn with_precision(&self, precision: u8) -> Result<FinMoneyCurrency> {
        if precision > 28 {
            return Err(MoneyError::InvalidPrecision(precision as u32));
        }

        Ok(FinMoneyCurrency {
            id: self.id,
            name: self.name,
            code: self.code,
            precision,
        })
    }

    /// Checks if this currency has the same ID as another currency.
    pub fn is_same_currency(&self, other: &FinMoneyCurrency) -> bool {
        self.id == other.id
    }

    // Helper methods for sanitization
    fn sanitize_and_parse_name(
        name: &str,
    ) -> std::result::Result<TinyAsciiStr<52>, tinystr::ParseError> {
        // Try to parse as-is first
        if let Ok(ascii_name) = name.parse() {
            return Ok(ascii_name);
        }

        // Sanitize by replacing non-ASCII characters with underscores
        let sanitized = name
            .chars()
            .map(|c| if c.is_ascii() { c } else { '_' })
            .collect::<String>();

        // Truncate if needed
        let truncated = if sanitized.len() > 52 {
            &sanitized[..52]
        } else {
            &sanitized
        };

        truncated.parse()
    }

    fn sanitize_and_parse_code(
        code: &str,
    ) -> std::result::Result<TinyAsciiStr<16>, tinystr::ParseError> {
        // Try to parse as-is first
        if let Ok(ascii_code) = code.parse() {
            return Ok(ascii_code);
        }

        // Sanitize by replacing non-ASCII characters with underscores
        let sanitized = code
            .chars()
            .map(|c| if c.is_ascii() { c } else { '_' })
            .collect::<String>();

        // Truncate if needed
        let truncated = if sanitized.len() > 16 {
            &sanitized[..16]
        } else {
            &sanitized
        };

        truncated.parse()
    }
}

// Common currency constants
impl FinMoneyCurrency {
    /// US Dollar with 2 decimal places precision.
    pub const USD: FinMoneyCurrency = FinMoneyCurrency {
        id: 1,
        name: None, // TinyAsciiStr doesn't support const construction with Some
        code: unsafe { TinyAsciiStr::from_utf8_unchecked(*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0") },
        precision: 2,
    };

    /// Euro with 2 decimal places precision.
    pub const EUR: FinMoneyCurrency = FinMoneyCurrency {
        id: 2,
        name: None,
        code: unsafe { TinyAsciiStr::from_utf8_unchecked(*b"EUR\0\0\0\0\0\0\0\0\0\0\0\0\0") },
        precision: 2,
    };

    /// Bitcoin with 8 decimal places precision.
    pub const BTC: FinMoneyCurrency = FinMoneyCurrency {
        id: 3,
        name: None,
        code: unsafe { TinyAsciiStr::from_utf8_unchecked(*b"BTC\0\0\0\0\0\0\0\0\0\0\0\0\0") },
        precision: 8,
    };

    /// Ethereum with 18 decimal places precision.
    pub const ETH: FinMoneyCurrency = FinMoneyCurrency {
        id: 4,
        name: None,
        code: unsafe { TinyAsciiStr::from_utf8_unchecked(*b"ETH\0\0\0\0\0\0\0\0\0\0\0\0\0") },
        precision: 18,
    };
}
