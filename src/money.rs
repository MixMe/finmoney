//! Core FinMoney type and operations.

use crate::{FinMoneyCurrency, FinMoneyError, FinMoneyRoundingStrategy};
use rust_decimal::{Decimal, MathematicalOps, RoundingStrategy};
use rust_decimal_macros::dec;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

/// Represents a monetary value with an amount and associated currency.
///
/// `FinMoney` ensures that all arithmetic operations are performed between compatible currencies
/// and provides precise decimal arithmetic suitable for financial calculations.
///
/// # Examples
///
/// ```rust
/// use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyError};
/// use rust_decimal_macros::dec;
///
/// let usd = FinMoneyCurrency::new(1, "USD", None::<String>, 2)?;
/// let price = FinMoney::new(dec!(10.50), usd);
/// let tax = FinMoney::new(dec!(1.05), usd);
/// let total = (price + tax)?;
///
/// assert_eq!(total.get_amount(), dec!(11.55));
/// # Ok::<(), FinMoneyError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FinMoney {
    amount: Decimal,
    currency: FinMoneyCurrency,
}

impl Default for FinMoney {
    /// Creates a zero-valued FinMoney with the default currency.
    fn default() -> Self {
        Self {
            amount: Decimal::ZERO,
            currency: FinMoneyCurrency::default(),
        }
    }
}

impl FinMoney {
    // -- Internal Helpers --

    #[inline]
    fn assert_same_currency(&self, other: Self) -> Result<(), FinMoneyError> {
        if !self.currency.is_same_currency(&other.currency) {
            return Err(FinMoneyError::CurrencyMismatch {
                expected: self.currency.get_code().to_string(),
                actual: other.currency.get_code().to_string(),
            });
        }
        Ok(())
    }

    #[inline]
    fn round_result(&self, value: Decimal, strategy: FinMoneyRoundingStrategy) -> Decimal {
        value.round_dp_with_strategy(
            self.currency.get_precision().into(),
            strategy.to_decimal_strategy(),
        )
    }

    // -- Constructors --

    /// Creates a new `FinMoney` with the given amount and currency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoneyCurrency::USD;
    /// let FinMoney = FinMoney::new(dec!(42.50), usd);
    /// assert_eq!(FinMoney.get_amount(), dec!(42.50));
    /// ```
    #[inline]
    pub fn new(amount: Decimal, currency: FinMoneyCurrency) -> Self {
        Self { amount, currency }
    }

    /// Creates a new `FinMoney` by rounding the provided amount to the currency's precision
    /// using the specified rounding strategy.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyRoundingStrategy};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoneyCurrency::USD; // 2 decimal places
    /// let FinMoney = FinMoney::new_with_precision(
    ///     dec!(42.567),
    ///     usd,
    ///     FinMoneyRoundingStrategy::MidpointNearestEven
    /// );
    /// assert_eq!(FinMoney.get_amount(), dec!(42.57));
    /// ```
    pub fn new_with_precision(
        amount: Decimal,
        currency: FinMoneyCurrency,
        strategy: FinMoneyRoundingStrategy,
    ) -> Self {
        let s = strategy.to_decimal_strategy();
        let rounded_amount = amount.round_dp_with_strategy(currency.get_precision().into(), s);
        Self {
            amount: rounded_amount,
            currency,
        }
    }

    /// Returns a `FinMoney` value of zero with the given currency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoneyCurrency::USD;
    /// let zero = FinMoney::zero(usd);
    /// assert_eq!(zero.get_amount(), dec!(0));
    /// assert!(zero.is_zero());
    /// ```
    #[inline]
    pub fn zero(currency: FinMoneyCurrency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    /// Creates a `FinMoney` from an `i64` value and a currency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let m = FinMoney::from_i64(100, FinMoneyCurrency::USD);
    /// assert_eq!(m.get_amount(), dec!(100));
    /// assert_eq!(m.get_currency(), FinMoneyCurrency::USD);
    /// ```
    #[inline]
    pub fn from_i64(value: i64, currency: FinMoneyCurrency) -> FinMoney {
        FinMoney::new(Decimal::from(value), currency)
    }

    /// Creates a `FinMoney` from an `f64` value and a currency.
    ///
    /// # Errors
    ///
    /// Returns `Err(FinMoneyError::InvalidAmount)` if the value is NaN or infinite.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyError};
    /// use rust_decimal_macros::dec;
    ///
    /// let m = FinMoney::from_f64(10.5, FinMoneyCurrency::USD)?;
    /// assert_eq!(m.get_amount(), dec!(10.5));
    ///
    /// assert!(FinMoney::from_f64(f64::NAN, FinMoneyCurrency::USD).is_err());
    /// assert!(FinMoney::from_f64(f64::INFINITY, FinMoneyCurrency::USD).is_err());
    /// # Ok::<(), FinMoneyError>(())
    /// ```
    pub fn from_f64(value: f64, currency: FinMoneyCurrency) -> Result<FinMoney, FinMoneyError> {
        Decimal::try_from(value)
            .map(|d| FinMoney::new(d, currency))
            .map_err(|_| FinMoneyError::InvalidAmount(format!("invalid f64: {}", value)))
    }

    // -- Accessors (getters) --

    /// Returns the amount of FinMoney as a `Decimal`.
    #[inline]
    pub fn get_amount(&self) -> Decimal {
        self.amount
    }

    /// Returns the currency of this FinMoney value.
    #[inline]
    pub fn get_currency(&self) -> FinMoneyCurrency {
        self.currency
    }

    /// Returns the currency identifier.
    #[inline]
    pub fn get_currency_id(&self) -> i32 {
        self.currency.get_id()
    }

    /// Returns the precision used for this FinMoney value.
    #[inline]
    pub fn get_precision(&self) -> u8 {
        self.currency.get_precision()
    }

    /// Returns the currency code.
    #[inline]
    pub fn get_currency_code(&self) -> &str {
        self.currency.get_code()
    }

    // -- Arithmetic Operations --

    /// Adds another `FinMoney` value to this one, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    /// Returns `FinMoneyError::ArithmeticOverflow` if the result overflows.
    pub fn plus_money(&self, other: FinMoney) -> Result<FinMoney, FinMoneyError> {
        self.assert_same_currency(other)?;
        self.amount
            .checked_add(other.amount)
            .map(|sum| FinMoney::new(sum, self.currency))
            .ok_or(FinMoneyError::ArithmeticOverflow)
    }

    /// Adds a `Decimal` amount to this `FinMoney`.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::ArithmeticOverflow` if the result overflows.
    #[inline]
    pub fn plus_decimal(&self, d: Decimal) -> Result<FinMoney, FinMoneyError> {
        self.amount
            .checked_add(d)
            .map(|sum| FinMoney::new(sum, self.currency))
            .ok_or(FinMoneyError::ArithmeticOverflow)
    }

    /// Subtracts another `FinMoney` value from this one, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    /// Returns `FinMoneyError::ArithmeticOverflow` if the result overflows.
    pub fn minus_money(&self, other: FinMoney) -> Result<FinMoney, FinMoneyError> {
        self.assert_same_currency(other)?;
        self.amount
            .checked_sub(other.amount)
            .map(|diff| FinMoney::new(diff, self.currency))
            .ok_or(FinMoneyError::ArithmeticOverflow)
    }

    /// Subtracts a `Decimal` amount from this `FinMoney`.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::ArithmeticOverflow` if the result overflows.
    #[inline]
    pub fn minus_decimal(&self, d: Decimal) -> Result<FinMoney, FinMoneyError> {
        self.amount
            .checked_sub(d)
            .map(|diff| FinMoney::new(diff, self.currency))
            .ok_or(FinMoneyError::ArithmeticOverflow)
    }

    /// Multiplies this `FinMoney` by another `FinMoney`, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    /// Returns `FinMoneyError::ArithmeticOverflow` if the result overflows.
    pub fn multiplied_by_money(&self, other: FinMoney) -> Result<FinMoney, FinMoneyError> {
        self.assert_same_currency(other)?;
        self.amount
            .checked_mul(other.amount)
            .map(|product| FinMoney::new(product, self.currency))
            .ok_or(FinMoneyError::ArithmeticOverflow)
    }

    /// Multiplies this `FinMoney` by a `Decimal`.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::ArithmeticOverflow` if the result overflows.
    #[inline]
    pub fn multiplied_by_decimal(&self, d: Decimal) -> Result<FinMoney, FinMoneyError> {
        self.amount
            .checked_mul(d)
            .map(|product| FinMoney::new(product, self.currency))
            .ok_or(FinMoneyError::ArithmeticOverflow)
    }

    /// Splits this `FinMoney` proportionally according to the given weights.
    ///
    /// The algorithm rounds each part toward zero, then distributes the remainder
    /// one minimum step at a time starting from the first part, guaranteeing that
    /// the sum of all parts equals the original amount.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::InvalidAmount` if all weights are zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyError};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoneyCurrency::USD;
    /// let total = FinMoney::new(dec!(100.00), usd);
    /// let parts = total.allocate(&[dec!(1), dec!(1), dec!(1)])?;
    /// assert_eq!(parts.len(), 3);
    /// let sum: rust_decimal::Decimal = parts.iter().map(|p| p.get_amount()).sum();
    /// assert_eq!(sum, dec!(100.00));
    /// # Ok::<(), FinMoneyError>(())
    /// ```
    pub fn allocate(&self, weights: &[Decimal]) -> Result<Vec<FinMoney>, FinMoneyError> {
        if weights.is_empty() {
            return Ok(vec![]);
        }
        let total_weight: Decimal = weights.iter().copied().sum();
        if total_weight.is_zero() {
            return Err(FinMoneyError::InvalidAmount("all weights are zero".into()));
        }
        let precision = self.currency.get_precision() as u32;
        let mut parts: Vec<Decimal> = weights
            .iter()
            .map(|w| {
                (self.amount * *w / total_weight)
                    .round_dp_with_strategy(precision, RoundingStrategy::ToZero)
            })
            .collect();
        let allocated: Decimal = parts.iter().copied().sum();
        let mut remainder = self.amount - allocated;
        let step = Decimal::new(1, precision);
        let mut i = 0;
        while remainder > Decimal::ZERO && i < parts.len() {
            parts[i] += step;
            remainder -= step;
            i += 1;
        }
        Ok(parts
            .iter()
            .map(|&p| FinMoney::new(p, self.currency))
            .collect())
    }

    /// Divides this `FinMoney` by another `FinMoney`, rounding according to the strategy.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    /// Returns `FinMoneyError::DivisionByZero` if the divisor is zero.
    pub fn divided_by_money(
        &self,
        other: FinMoney,
        round_strategy: FinMoneyRoundingStrategy,
    ) -> Result<FinMoney, FinMoneyError> {
        self.assert_same_currency(other)?;
        if other.amount.is_zero() {
            return Err(FinMoneyError::DivisionByZero);
        }
        let raw = self.amount / other.amount;
        let rounded = self.round_result(raw, round_strategy);
        Ok(FinMoney::new(rounded, self.currency))
    }

    /// Divides this `FinMoney` by a `Decimal`, rounding according to the strategy.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::DivisionByZero` if the divisor is zero.
    pub fn divided_by_decimal(
        &self,
        d: Decimal,
        round_strategy: FinMoneyRoundingStrategy,
    ) -> Result<FinMoney, FinMoneyError> {
        if d.is_zero() {
            return Err(FinMoneyError::DivisionByZero);
        }
        let raw = self.amount / d;
        let rounded = self.round_result(raw, round_strategy);
        Ok(FinMoney::new(rounded, self.currency))
    }

    // -- Comparison Operations --

    /// Compares this `FinMoney` with another, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn compare(&self, other: FinMoney) -> Result<Ordering, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.amount.cmp(&other.amount))
    }

    /// Returns the minimum of self and other, ensuring same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn min(&self, other: FinMoney) -> Result<FinMoney, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(if self.amount <= other.amount {
            *self
        } else {
            other
        })
    }

    /// Returns the maximum of self and other, ensuring same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn max(&self, other: FinMoney) -> Result<FinMoney, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(if self.amount >= other.amount {
            *self
        } else {
            other
        })
    }

    /// Checks if this `FinMoney` has the same currency as another.
    pub fn is_same_currency(&self, other: FinMoney) -> bool {
        self.currency.is_same_currency(&other.currency)
    }

    /// Checks if this `FinMoney` is equal to another in both amount and currency.
    pub fn is_equal_to(&self, other: FinMoney) -> bool {
        self.currency.is_same_currency(&other.currency) && self.amount == other.amount
    }

    /// Checks if this `FinMoney` is less than another, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn is_less_than(&self, other: FinMoney) -> Result<bool, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.amount < other.amount)
    }

    /// Checks if this `FinMoney` is less than or equal to another, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn is_less_than_or_equal(&self, other: FinMoney) -> Result<bool, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.amount <= other.amount)
    }

    /// Checks if this `FinMoney` amount is less than a `Decimal`.
    pub fn is_less_than_decimal(&self, decimal: Decimal) -> bool {
        self.amount < decimal
    }

    /// Checks if this `FinMoney` amount is less than or equal to a `Decimal`.
    pub fn is_less_than_or_equal_decimal(&self, decimal: Decimal) -> bool {
        self.amount <= decimal
    }

    /// Checks if this `FinMoney` is greater than another, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn is_greater_than(&self, other: FinMoney) -> Result<bool, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.amount > other.amount)
    }

    /// Checks if this `FinMoney` is greater than or equal to another, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn is_greater_than_or_equal(&self, other: FinMoney) -> Result<bool, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.amount >= other.amount)
    }

    /// Checks if this `FinMoney` amount is greater than a `Decimal`.
    pub fn is_greater_than_decimal(&self, decimal: Decimal) -> bool {
        self.amount > decimal
    }

    /// Checks if this `FinMoney` amount is greater than or equal to a `Decimal`.
    pub fn is_greater_than_or_equal_decimal(&self, decimal: Decimal) -> bool {
        self.amount >= decimal
    }

    // -- Rounding and Scaling --

    /// Rescales the amount to a new precision.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::InvalidPrecision` if the new precision is > 28.
    pub fn rescale(&self, new_precision: u8) -> Result<FinMoney, FinMoneyError> {
        let new_currency = self.currency.with_precision(new_precision)?;
        let scaled = self.amount.round_dp(new_precision.into());
        Ok(FinMoney::new(scaled, new_currency))
    }

    /// Returns a rounded version of this `FinMoney` using the specified strategy.
    pub fn rounded(&self, strategy: FinMoneyRoundingStrategy) -> FinMoney {
        let amount = self.round_result(self.amount, strategy);
        FinMoney::new(amount, self.currency)
    }

    /// Returns the largest integer less than or equal to this `FinMoney`.
    #[inline]
    pub fn floor(&self) -> FinMoney {
        FinMoney::new(self.amount.floor(), self.currency)
    }

    /// Returns the smallest integer greater than or equal to this `FinMoney`.
    #[inline]
    pub fn ceil(&self) -> FinMoney {
        FinMoney::new(self.amount.ceil(), self.currency)
    }

    /// Returns the integer part of this `FinMoney`, removing the fractional part.
    #[inline]
    pub fn trunc(&self) -> FinMoney {
        FinMoney::new(self.amount.trunc(), self.currency)
    }

    // -- Properties and Checks --

    /// Checks if the amount is an integer (no fractional part).
    #[inline]
    pub fn is_integer(&self) -> bool {
        self.amount.fract().is_zero()
    }

    /// Checks if the amount has a fractional part.
    #[inline]
    pub fn has_fraction(&self) -> bool {
        !self.amount.fract().is_zero()
    }

    /// Checks if the amount is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    /// Checks if the amount is positive (greater than zero).
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.amount.is_sign_positive() && !self.amount.is_zero()
    }

    /// Checks if the amount is negative (less than zero).
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.amount.is_sign_negative() && !self.amount.is_zero()
    }

    /// Checks if the amount is positive or zero.
    #[inline]
    pub fn is_positive_or_zero(&self) -> bool {
        self.amount.is_sign_positive()
    }

    /// Checks if the amount is negative or zero.
    #[inline]
    pub fn is_negative_or_zero(&self) -> bool {
        self.amount.is_sign_negative() || self.amount.is_zero()
    }

    // -- Utilities --

    /// Returns the square root of the amount.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::InvalidAmount` if the amount is negative.
    #[inline]
    pub fn sqrt(&self) -> Result<FinMoney, FinMoneyError> {
        match self.amount.sqrt() {
            Some(result) => Ok(FinMoney::new(result, self.currency)),
            None => Err(FinMoneyError::InvalidAmount(
                "cannot take square root of negative number".to_string(),
            )),
        }
    }

    /// Returns the absolute value of the amount.
    #[inline]
    pub fn abs(&self) -> FinMoney {
        FinMoney::new(self.amount.abs(), self.currency)
    }

    /// Returns the negated value of the amount.
    #[inline]
    pub fn negated(&self) -> FinMoney {
        FinMoney::new(-self.amount, self.currency)
    }

    /// Returns a normalized version of the amount.
    #[inline]
    pub fn normalize(&self) -> FinMoney {
        FinMoney::new(self.amount.normalize(), self.currency)
    }

    // -- Percentage Operations --

    /// Calculates the percentage change from the initial FinMoney to this FinMoney value.
    /// Returns the change as a Decimal percentage.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if currencies don't match.
    /// Returns `FinMoneyError::DivisionByZero` if initial amount is zero.
    pub fn percent_change_from(&self, initial: FinMoney) -> Result<Decimal, FinMoneyError> {
        self.assert_same_currency(initial)?;

        if initial.amount.is_zero() {
            return Err(FinMoneyError::DivisionByZero);
        }

        Ok(((self.amount - initial.amount) * dec!(100)) / initial.amount)
    }

    /// Calculates the negative percentage change from the initial FinMoney to this FinMoney value.
    /// Returns the negative change as a Decimal percentage.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if currencies don't match.
    /// Returns `FinMoneyError::DivisionByZero` if initial amount is zero.
    pub fn negative_percent_change_from(
        &self,
        initial: FinMoney,
    ) -> Result<Decimal, FinMoneyError> {
        self.assert_same_currency(initial)?;

        if initial.amount.is_zero() {
            return Err(FinMoneyError::DivisionByZero);
        }

        Ok(((initial.amount - self.amount) * dec!(100)) / initial.amount)
    }

    /// Static method to calculate percentage change between two FinMoney values.
    /// Returns the change as a Decimal percentage.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if currencies don't match.
    /// Returns `FinMoneyError::DivisionByZero` if initial amount is zero.
    pub fn percent_change(
        initial: FinMoney,
        new_value: FinMoney,
    ) -> Result<Decimal, FinMoneyError> {
        new_value.percent_change_from(initial)
    }

    /// Static method to calculate negative percentage change between two FinMoney values.
    /// Returns the negative change as a Decimal percentage.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if currencies don't match.
    /// Returns `FinMoneyError::DivisionByZero` if initial amount is zero.
    pub fn negative_percent_change(
        initial: FinMoney,
        new_value: FinMoney,
    ) -> Result<Decimal, FinMoneyError> {
        new_value.negative_percent_change_from(initial)
    }

    // -- Precision Operations --

    /// Rounds the amount to `dp` decimal places using the provided rounding strategy.
    pub fn round_dp_with_strategy(&self, dp: u32, strategy: FinMoneyRoundingStrategy) -> FinMoney {
        let s = strategy.to_decimal_strategy();
        let rounded = self.amount.round_dp_with_strategy(dp, s);
        FinMoney::new(rounded, self.currency)
    }

    /// Rounds the amount to `dp` decimal places using the default rounding strategy.
    pub fn round_dp(&self, dp: u32) -> FinMoney {
        let rounded = self.amount.round_dp(dp);
        FinMoney::new(rounded, self.currency)
    }

    // -- Unchecked Arithmetic (for hot paths where currency match is guaranteed) --

    /// Adds another `FinMoney` without checking currency match or overflow.
    ///
    /// # Panics
    ///
    /// Panics if currencies differ or if addition overflows.
    /// Use this only when the currency match is guaranteed by construction
    /// (e.g., both values originate from the same balance column).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoneyCurrency::USD;
    /// let a = FinMoney::new(dec!(10), usd);
    /// let b = FinMoney::new(dec!(20), usd);
    /// let total = a.unchecked_plus(b);
    /// assert_eq!(total.get_amount(), dec!(30));
    /// ```
    #[inline]
    pub fn unchecked_plus(self, other: FinMoney) -> FinMoney {
        self.plus_money(other)
            .expect("unchecked_plus: currency mismatch or overflow")
    }

    /// Subtracts another `FinMoney` without checking currency match or overflow.
    ///
    /// # Panics
    ///
    /// Panics if currencies differ or if subtraction overflows.
    /// Use this only when the currency match is guaranteed by construction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoneyCurrency::USD;
    /// let a = FinMoney::new(dec!(30), usd);
    /// let b = FinMoney::new(dec!(10), usd);
    /// let diff = a.unchecked_minus(b);
    /// assert_eq!(diff.get_amount(), dec!(20));
    /// ```
    #[inline]
    pub fn unchecked_minus(self, other: FinMoney) -> FinMoney {
        self.minus_money(other)
            .expect("unchecked_minus: currency mismatch or overflow")
    }

    /// Multiplies this `FinMoney` by a `Decimal` without overflow checking.
    ///
    /// # Panics
    ///
    /// Panics if multiplication overflows.
    #[inline]
    pub fn unchecked_mul(self, d: Decimal) -> FinMoney {
        self.multiplied_by_decimal(d)
            .expect("unchecked_mul: overflow")
    }
}
// -- Tick Operations --

impl FinMoney {
    /// Rounds the amount to the nearest allowed tick size.
    /// Works for any tick sizes: 0.001, 0.25, 9, 10, 101, etc.
    ///
    /// # Arguments
    ///
    /// * `tick` - The tick size to round to (must be positive)
    /// * `strategy` - The rounding strategy to use
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::InvalidTick` if tick is zero or negative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyRoundingStrategy};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoneyCurrency::USD;
    /// let price = FinMoney::new(dec!(10.567), usd);
    ///
    /// // Round to nearest 0.25
    /// let rounded = price.to_tick(dec!(0.25), FinMoneyRoundingStrategy::MidpointNearestEven)?;
    /// assert_eq!(rounded.get_amount(), dec!(10.50));
    /// # Ok::<(), finmoney::FinMoneyError>(())
    /// ```
    pub fn to_tick(
        &self,
        tick: Decimal,
        strategy: FinMoneyRoundingStrategy,
    ) -> Result<FinMoney, FinMoneyError> {
        if tick <= Decimal::ZERO {
            return Err(FinMoneyError::InvalidTick);
        }
        // Normalize tick to remove trailing zeros (e.g. 0.000100000000 → 0.0001).
        // Without this, tick_power10_dp fails for ticks with trailing zeros
        // because scale() returns the raw scale, not the significant scale.
        let tick = tick.normalize();
        let s = strategy.to_decimal_strategy();
        // Fast path: if tick is a power of 10 (like 0.001), just round to decimal places
        if let Some(dp) = Self::tick_power10_dp(tick) {
            let amt = self.amount.round_dp_with_strategy(dp, s);
            return Ok(FinMoney::new(amt, self.currency));
        }
        // General path: k = amount / tick → round k to integer → multiply back
        let k = self.amount / tick;
        let k_rounded = k.round_dp_with_strategy(0, s);
        let amt = k_rounded * tick;
        Ok(FinMoney::new(amt, self.currency))
    }

    /// Rounds down to the nearest tick size (floor).
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::InvalidTick` if tick is zero or negative.
    pub fn to_tick_down(&self, tick: Decimal) -> Result<FinMoney, FinMoneyError> {
        self.to_tick(tick, FinMoneyRoundingStrategy::ToNegativeInfinity)
    }

    /// Rounds up to the nearest tick size (ceiling).
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::InvalidTick` if tick is zero or negative.
    pub fn to_tick_up(&self, tick: Decimal) -> Result<FinMoney, FinMoneyError> {
        self.to_tick(tick, FinMoneyRoundingStrategy::ToPositiveInfinity)
    }

    /// Rounds to the nearest tick size using banker's rounding.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::InvalidTick` if tick is zero or negative.
    pub fn to_tick_nearest(&self, tick: Decimal) -> Result<FinMoney, FinMoneyError> {
        self.to_tick(tick, FinMoneyRoundingStrategy::MidpointNearestEven)
    }

    /// Checks if the amount is a multiple of the given tick size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoneyCurrency::USD;
    /// let price = FinMoney::new(dec!(10.50), usd);
    ///
    /// assert!(price.is_multiple_of_tick(dec!(0.25)));
    /// assert!(!price.is_multiple_of_tick(dec!(0.33)));
    /// ```
    pub fn is_multiple_of_tick(&self, tick: Decimal) -> bool {
        if tick.is_zero() {
            return false;
        }
        // Normalize to strip trailing zeros (same reasoning as to_tick)
        let tick = tick.normalize();

        // For power-of-ten ticks, check if rounding to dp places equals original
        if let Some(dp) = Self::tick_power10_dp(tick) {
            let amt = self.amount.round_dp(dp);
            return amt == self.amount;
        }

        // General case: check if amount/tick is an integer
        let k = self.amount / tick;
        k.fract().is_zero()
    }

    /// Helper function: if tick == 10^-dp (e.g., 0.001 → dp=3), return dp.
    #[inline]
    fn tick_power10_dp(tick: Decimal) -> Option<u32> {
        // If tick is exactly 10^-dp, then its scale is dp and its coefficient is 1.
        // This avoids powi/multiply allocations and is significantly cheaper.
        let dp = tick.scale();
        if tick == Decimal::new(1, dp) {
            Some(dp)
        } else {
            None
        }
    }

    /// Safe alternative to `Sum` — sums an iterator of `FinMoney` returning `Result`.
    ///
    /// Returns `Err(InvalidAmount)` for an empty iterator, or propagates
    /// `CurrencyMismatch` / `ArithmeticOverflow` from `plus_money`.
    ///
    /// # Errors
    ///
    /// - `FinMoneyError::InvalidAmount` if the iterator is empty.
    /// - `FinMoneyError::CurrencyMismatch` if currencies differ.
    /// - `FinMoneyError::ArithmeticOverflow` if the sum overflows.
    ///
    /// # Examples
    ///
    /// ```
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let items = vec![
    ///     FinMoney::new(dec!(10), FinMoneyCurrency::USD),
    ///     FinMoney::new(dec!(20), FinMoneyCurrency::USD),
    /// ];
    /// let total = FinMoney::try_sum(items.into_iter()).unwrap();
    /// assert_eq!(total.get_amount(), dec!(30));
    /// ```
    pub fn try_sum(iter: impl Iterator<Item = FinMoney>) -> Result<FinMoney, FinMoneyError> {
        let mut iter = iter.peekable();
        let first = iter
            .next()
            .ok_or(FinMoneyError::InvalidAmount("empty iterator".into()))?;
        iter.try_fold(first, |acc, item| acc.plus_money(item))
    }

    /// Converts this monetary value to a different currency using the given exchange rate.
    ///
    /// The result is rounded to the target currency's precision using the specified
    /// rounding strategy.
    ///
    /// # Errors
    ///
    /// - `FinMoneyError::InvalidAmount` if `rate` is zero or negative.
    /// - `FinMoneyError::ArithmeticOverflow` if the multiplication overflows.
    ///
    /// # Examples
    ///
    /// ```
    /// use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyRoundingStrategy};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoney::new(dec!(100), FinMoneyCurrency::USD);
    /// let eur = usd.convert_to(
    ///     FinMoneyCurrency::EUR,
    ///     dec!(0.85),
    ///     FinMoneyRoundingStrategy::MidpointNearestEven,
    /// ).unwrap();
    /// assert_eq!(eur.get_amount(), dec!(85.00));
    /// assert_eq!(eur.get_currency(), FinMoneyCurrency::EUR);
    /// ```
    pub fn convert_to(
        &self,
        target_currency: FinMoneyCurrency,
        rate: Decimal,
        strategy: FinMoneyRoundingStrategy,
    ) -> Result<FinMoney, FinMoneyError> {
        if rate <= Decimal::ZERO {
            return Err(FinMoneyError::InvalidAmount(
                "exchange rate must be positive".into(),
            ));
        }
        let converted = self
            .amount
            .checked_mul(rate)
            .ok_or(FinMoneyError::ArithmeticOverflow)?;
        let precision = target_currency.get_precision() as u32;
        let rounded = converted.round_dp_with_strategy(precision, strategy.to_decimal_strategy());
        Ok(FinMoney::new(rounded, target_currency))
    }

    /// Computes the exchange rate from `self` to `other`: `other.amount / self.amount`.
    ///
    /// # Errors
    ///
    /// - `FinMoneyError::DivisionByZero` if `self.amount` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let usd = FinMoney::new(dec!(100), FinMoneyCurrency::USD);
    /// let eur = FinMoney::new(dec!(85), FinMoneyCurrency::EUR);
    /// let rate = usd.exchange_rate_to(eur).unwrap();
    /// assert_eq!(rate, dec!(0.85));
    /// ```
    pub fn exchange_rate_to(&self, other: FinMoney) -> Result<Decimal, FinMoneyError> {
        if self.amount.is_zero() {
            return Err(FinMoneyError::DivisionByZero);
        }
        Ok(other.amount / self.amount)
    }

    /// Formats the monetary value with custom thousands and decimal separators,
    /// appending the currency code.
    ///
    /// Inserts `thousands_sep` every 3 digits in the integer part (right to left),
    /// uses `decimal_sep` between integer and fractional parts, and appends ` CODE`.
    ///
    /// # Examples
    ///
    /// ```
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let m = FinMoney::new(dec!(1234567.89), FinMoneyCurrency::USD);
    /// assert_eq!(m.format_with_separator(',', '.'), "1,234,567.89 USD");
    ///
    /// let neg = FinMoney::new(dec!(-50000), FinMoneyCurrency::EUR);
    /// assert_eq!(neg.format_with_separator('.', ','), "-50.000 EUR");
    /// ```
    pub fn format_with_separator(&self, thousands_sep: char, decimal_sep: char) -> String {
        let s = self.amount.to_string();
        let (negative, abs_str) = if let Some(rest) = s.strip_prefix('-') {
            (true, rest)
        } else {
            (false, s.as_str())
        };

        let (int_part, frac_part) = match abs_str.split_once('.') {
            Some((i, f)) => (i, Some(f)),
            None => (abs_str, None),
        };

        // Insert thousands separator every 3 digits from the right
        let mut formatted_int = String::with_capacity(int_part.len() + int_part.len() / 3);
        for (i, ch) in int_part.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                formatted_int.push(thousands_sep);
            }
            formatted_int.push(ch);
        }
        let formatted_int: String = formatted_int.chars().rev().collect();

        let mut result = String::new();
        if negative {
            result.push('-');
        }
        result.push_str(&formatted_int);

        if let Some(frac) = frac_part {
            result.push(decimal_sep);
            result.push_str(frac);
        }

        result.push(' ');
        result.push_str(self.currency.get_code());
        result
    }

    /// Formats the amount padded with trailing zeros to exactly `dp` decimal places,
    /// followed by the currency code.
    ///
    /// # Examples
    ///
    /// ```
    /// use finmoney::{FinMoney, FinMoneyCurrency};
    /// use rust_decimal_macros::dec;
    ///
    /// let m = FinMoney::new(dec!(10.5), FinMoneyCurrency::USD);
    /// assert_eq!(m.format_padded(4), "10.5000 USD");
    ///
    /// let m2 = FinMoney::new(dec!(42), FinMoneyCurrency::EUR);
    /// assert_eq!(m2.format_padded(0), "42 EUR");
    ///
    /// let m3 = FinMoney::new(dec!(-7.123456), FinMoneyCurrency::BTC);
    /// assert_eq!(m3.format_padded(3), "-7.123 BTC");
    /// ```
    pub fn format_padded(&self, dp: u32) -> String {
        let rounded = self
            .amount
            .round_dp_with_strategy(dp, rust_decimal::RoundingStrategy::MidpointNearestEven);

        let s = rounded.to_string();

        let result = if dp == 0 {
            // No decimal places — strip any fractional part
            match s.split_once('.') {
                Some((int_part, _)) => int_part.to_string(),
                None => s,
            }
        } else {
            match s.split_once('.') {
                Some((int_part, frac_part)) => {
                    let padded_frac = if (frac_part.len() as u32) < dp {
                        let zeros = dp as usize - frac_part.len();
                        format!("{}{}", frac_part, "0".repeat(zeros))
                    } else {
                        frac_part[..dp as usize].to_string()
                    };
                    format!("{}.{}", int_part, padded_frac)
                }
                None => {
                    // No decimal point — add one with zeros
                    format!("{}.{}", s, "0".repeat(dp as usize))
                }
            }
        };

        format!("{} {}", result, self.currency.get_code())
    }
}

// -- Operator Overloads --

impl Add for FinMoney {
    type Output = Result<FinMoney, FinMoneyError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.plus_money(rhs)
    }
}

impl Sub for FinMoney {
    type Output = Result<FinMoney, FinMoneyError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.minus_money(rhs)
    }
}

impl Mul<Decimal> for FinMoney {
    type Output = Result<FinMoney, FinMoneyError>;

    fn mul(self, rhs: Decimal) -> Self::Output {
        self.multiplied_by_decimal(rhs)
    }
}

impl Mul<FinMoney> for Decimal {
    type Output = Result<FinMoney, FinMoneyError>;

    fn mul(self, rhs: FinMoney) -> Self::Output {
        rhs.multiplied_by_decimal(self)
    }
}

impl AddAssign for FinMoney {
    /// Adds another `FinMoney` in place.
    ///
    /// # Panics
    ///
    /// Panics if currencies differ or if addition overflows.
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus_money(rhs).expect("AddAssign: currency mismatch or overflow");
    }
}

impl SubAssign for FinMoney {
    /// Subtracts another `FinMoney` in place.
    ///
    /// # Panics
    ///
    /// Panics if currencies differ or if subtraction overflows.
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus_money(rhs).expect("SubAssign: currency mismatch or overflow");
    }
}

impl Neg for FinMoney {
    type Output = FinMoney;

    fn neg(self) -> Self::Output {
        self.negated()
    }
}

impl fmt::Display for FinMoney {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency.get_code())
    }
}

impl TryFrom<(f64, FinMoneyCurrency)> for FinMoney {
    type Error = FinMoneyError;

    fn try_from((value, currency): (f64, FinMoneyCurrency)) -> Result<Self, Self::Error> {
        FinMoney::from_f64(value, currency)
    }
}

impl std::iter::Sum for FinMoney {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let first = iter.next().expect("cannot sum empty iterator of FinMoney");
        iter.fold(first, |acc, item| {
            (acc + item).expect("currency mismatch or overflow in Sum")
        })
    }
}
