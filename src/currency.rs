//! Currency representation and management.

use crate::error::{FinMoneyError, Result};
use std::fmt;
use tinystr::{TinyAsciiStr, tinystr};

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
    const UNDEFINED_CODE: TinyAsciiStr<16> = tinystr!(16, "UNDEFINED");
    const INVALID_CODE: TinyAsciiStr<16> = tinystr!(16, "INVALID");

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
    /// let usd = FinMoneyCurrency::new(1, "USD", Some("US Dollar"), 2)?;
    /// let btc = FinMoneyCurrency::new(2, "BTC", Some("Bitcoin"), 8)?;
    /// # Ok::<(), finmoney::FinMoneyError>(())
    /// ```
    pub fn new(
        id: i32,
        code: impl Into<String>,
        name: Option<impl Into<String>>,
        precision: u8,
    ) -> Result<FinMoneyCurrency> {
        if precision > 28 {
            return Err(FinMoneyError::InvalidPrecision(precision as u32));
        }
        let code = code.into();
        let parsed_name = match name {
            Some(n) => {
                let n = n.into();
                match Self::sanitize_and_parse_name(&n) {
                    Ok(ascii_name) => Some(ascii_name),
                    Err(_) => return Err(FinMoneyError::InvalidCurrencyName(n)),
                }
            }
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
    #[inline]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Returns the human-readable name of this currency, if available.
    #[inline]
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    /// Returns the currency code (e.g., "USD", "EUR").
    #[inline]
    pub fn get_code(&self) -> &str {
        self.code.as_str()
    }

    /// Returns the currency code as a `TinyAsciiStr<16>` for zero-copy usage.
    #[inline]
    pub fn get_code_tiny(&self) -> TinyAsciiStr<16> {
        self.code
    }

    /// Returns the currency name as a `TinyAsciiStr<52>`, if available.
    #[inline]
    pub fn get_name_tiny(&self) -> Option<TinyAsciiStr<52>> {
        self.name
    }

    /// Returns the precision (number of decimal places) for this currency.
    #[inline]
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
    #[inline]
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
        name: None,
        code: tinystr!(16, "USD"),
        precision: 2,
    };

    /// Euro with 2 decimal places precision.
    pub const EUR: FinMoneyCurrency = FinMoneyCurrency {
        id: 2,
        name: None,
        code: tinystr!(16, "EUR"),
        precision: 2,
    };

    /// Bitcoin with 8 decimal places precision.
    pub const BTC: FinMoneyCurrency = FinMoneyCurrency {
        id: 3,
        name: None,
        code: tinystr!(16, "BTC"),
        precision: 8,
    };

    /// Ethereum with 18 decimal places precision.
    pub const ETH: FinMoneyCurrency = FinMoneyCurrency {
        id: 4,
        name: None,
        code: tinystr!(16, "ETH"),
        precision: 18,
    };

    /// British Pound with 2 decimal places precision.
    pub const GBP: FinMoneyCurrency = FinMoneyCurrency {
        id: 5,
        name: None,
        code: tinystr!(16, "GBP"),
        precision: 2,
    };

    /// Japanese Yen with 0 decimal places precision.
    pub const JPY: FinMoneyCurrency = FinMoneyCurrency {
        id: 6,
        name: None,
        code: tinystr!(16, "JPY"),
        precision: 0,
    };

    /// Swiss Franc with 2 decimal places precision.
    pub const CHF: FinMoneyCurrency = FinMoneyCurrency {
        id: 7,
        name: None,
        code: tinystr!(16, "CHF"),
        precision: 2,
    };

    /// Chinese Yuan with 2 decimal places precision.
    pub const CNY: FinMoneyCurrency = FinMoneyCurrency {
        id: 8,
        name: None,
        code: tinystr!(16, "CNY"),
        precision: 2,
    };

    /// Russian Ruble with 2 decimal places precision.
    pub const RUB: FinMoneyCurrency = FinMoneyCurrency {
        id: 9,
        name: None,
        code: tinystr!(16, "RUB"),
        precision: 2,
    };

    /// Tether with 6 decimal places precision.
    pub const USDT: FinMoneyCurrency = FinMoneyCurrency {
        id: 10,
        name: None,
        code: tinystr!(16, "USDT"),
        precision: 6,
    };

    /// Solana with 9 decimal places precision.
    pub const SOL: FinMoneyCurrency = FinMoneyCurrency {
        id: 11,
        name: None,
        code: tinystr!(16, "SOL"),
        precision: 9,
    };

    /// Returns a static slice of all predefined currencies.
    ///
    /// The list includes: USD, EUR, BTC, ETH, GBP, JPY, CHF, CNY, RUB, USDT, SOL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::FinMoneyCurrency;
    ///
    /// let currencies = FinMoneyCurrency::all_predefined();
    /// assert_eq!(currencies.len(), 11);
    /// assert_eq!(currencies[0], FinMoneyCurrency::USD);
    /// ```
    pub fn all_predefined() -> &'static [FinMoneyCurrency] {
        static ALL: [FinMoneyCurrency; 11] = [
            FinMoneyCurrency::USD,
            FinMoneyCurrency::EUR,
            FinMoneyCurrency::BTC,
            FinMoneyCurrency::ETH,
            FinMoneyCurrency::GBP,
            FinMoneyCurrency::JPY,
            FinMoneyCurrency::CHF,
            FinMoneyCurrency::CNY,
            FinMoneyCurrency::RUB,
            FinMoneyCurrency::USDT,
            FinMoneyCurrency::SOL,
        ];
        &ALL
    }
}

/// Displays the currency as `"CODE (Name)"` when a name is set,
/// or just `"CODE"` otherwise.
///
/// # Examples
///
/// ```
/// use finmoney::{FinMoneyCurrency, FinMoneyError};
///
/// // Predefined constant without a name
/// let usd = FinMoneyCurrency::USD;
/// assert_eq!(usd.to_string(), "USD");
///
/// // Custom currency with a name
/// let custom = FinMoneyCurrency::new(99, "XYZ", Some("My Token"), 2)?;
/// assert_eq!(custom.to_string(), "XYZ (My Token)");
/// # Ok::<(), FinMoneyError>(())
/// ```
impl fmt::Display for FinMoneyCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_name() {
            Some(name) => write!(f, "{} ({})", self.get_code(), name),
            None => write!(f, "{}", self.get_code()),
        }
    }
}
