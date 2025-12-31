//! Currency representation and management.

use crate::error::{FinMoneyError, Result};
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
            code: FinMoneyCurrency::UNDEFINED_CODE,
            precision: 8,
        }
    }
}

impl FinMoneyCurrency {
    // Common internal codes to avoid repeated parsing/allocations.
    const UNDEFINED_CODE: TinyAsciiStr<16> = unsafe {
        TinyAsciiStr::from_utf8_unchecked(*b"UNDEFINED\0\0\0\0\0\0\0")
    };
    const INVALID_CODE: TinyAsciiStr<16> = unsafe {
        TinyAsciiStr::from_utf8_unchecked(*b"INVALID\0\0\0\0\0\0\0\0\0")
    };

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
    /// Returns `FinMoneyError::InvalidPrecision` if precision > 28.
    /// Returns `FinMoneyError::InvalidCurrencyCode` if the code is invalid.
    /// Returns `FinMoneyError::InvalidCurrencyName` if the name is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::FinMoneyCurrency;
    ///
    /// let usd = FinMoneyCurrency::new(1, "USD".to_string(), Some("US Dollar".to_string()), 2)?;
    /// let btc = FinMoneyCurrency::new(2, "BTC".to_string(), Some("Bitcoin".to_string()), 8)?;
    /// # Ok::<(), finmoney::FinMoneyError>(())
    /// ```
    pub fn new(
        id: i32,
        code: impl Into<String>,
        name: Option<String>,
        precision: u8,
    ) -> Result<FinMoneyCurrency> {
        if precision > 28 {
            return Err(FinMoneyError::InvalidPrecision(precision as u32));
        }
        let code = code.into();
        let parsed_name = match name {
            Some(n) => match Self::sanitize_and_parse_name(&n) {
                Ok(ascii_name) => Some(ascii_name),
                Err(_) => return Err(FinMoneyError::InvalidCurrencyName(n)),
            },
            None => None,
        };

        let parsed_code = Self::sanitize_and_parse_code(code.as_str())
            .map_err(|_| FinMoneyError::InvalidCurrencyCode(code))?;

        Ok(Self {
            id,
            name: parsed_name,
            code: parsed_code,
            precision,
        })
    }

    /// Creates a new currency using pre-calculated `TinyAsciiStr` values.
    ///
    /// This method is more efficient than `new()` when you already have `TinyAsciiStr` values,
    /// as it avoids string parsing and sanitization. Use this when working with pre-validated
    /// currency data or when performance is critical.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the currency
    /// * `code` - Pre-calculated currency code as `TinyAsciiStr<16>`
    /// * `name` - Optional pre-calculated human-readable name as `TinyAsciiStr<52>`
    /// * `precision` - Number of decimal places (must be <= 28)
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::InvalidPrecision` if precision > 28.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::FinMoneyCurrency;
    /// use tinystr::TinyAsciiStr;
    ///
    /// let code: TinyAsciiStr<16> = "USD".parse().unwrap();
    /// let name: TinyAsciiStr<52> = "US Dollar".parse().unwrap();
    /// let usd = FinMoneyCurrency::new_from_tiny(1, code, Some(name), 2)?;
    /// # Ok::<(), finmoney::FinMoneyError>(())
    /// ```
    pub fn new_from_tiny(
        id: i32,
        code: TinyAsciiStr<16>,
        name: Option<TinyAsciiStr<52>>,
        precision: u8,
    ) -> Result<FinMoneyCurrency> {
        if precision > 28 {
            return Err(FinMoneyError::InvalidPrecision(precision as u32));
        }

        Ok(Self {
            id,
            name,
            code,
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
            Self::sanitize_and_parse_code(&code).unwrap_or(FinMoneyCurrency::INVALID_CODE);

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
        self.name.as_ref().map(|s| s.as_str())
    }

    /// Returns the currency code (e.g., "USD", "EUR").
    pub fn get_code(&self) -> &str {
        self.code.as_str()
    }

    /// Returns the precision (number of decimal places) for this currency.
    pub fn get_precision(&self) -> u8 {
        self.precision
    }

    /// Creates a new currency with the same properties but different precision.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::InvalidPrecision` if precision > 28.
    pub fn with_precision(&self, precision: u8) -> Result<FinMoneyCurrency> {
        if precision > 28 {
            return Err(FinMoneyError::InvalidPrecision(precision as u32));
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
    #[inline]
    fn sanitize_ascii_truncate(input: &str, max_len: usize) -> String {
        // Build only up to `max_len` chars; replace any non-ASCII char with '_'.
        // This avoids allocating/collecting the full string for long inputs.
        let mut out = String::with_capacity(std::cmp::min(input.len(), max_len));
        for (count, ch) in input.chars().enumerate() {
            if count == max_len {
                break;
            }
            out.push(if ch.is_ascii() { ch } else { '_' });
        }
        out
    }

    fn sanitize_and_parse_name(
        name: &str,
    ) -> std::result::Result<TinyAsciiStr<52>, tinystr::ParseError> {
        // Try to parse as-is first
        if let Ok(ascii_name) = name.parse() {
            return Ok(ascii_name);
        }

        let sanitized = Self::sanitize_ascii_truncate(name, 52);
        sanitized.parse()
    }

    fn sanitize_and_parse_code(
        code: &str,
    ) -> std::result::Result<TinyAsciiStr<16>, tinystr::ParseError> {
        // Try to parse as-is first
        if let Ok(ascii_code) = code.parse() {
            return Ok(ascii_code);
        }

        let sanitized = Self::sanitize_ascii_truncate(code, 16);
        sanitized.parse()
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
