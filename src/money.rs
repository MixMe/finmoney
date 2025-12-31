//! Core FinMoney type and operations.

use crate::{FinMoneyCurrency, FinMoneyError, FinMoneyRoundingStrategy};
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

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
/// let usd = FinMoneyCurrency::new(1, "USD".to_string(), None, 2)?;
/// let price = FinMoney::new(dec!(10.50), usd);
/// let tax = FinMoney::new(dec!(1.05), usd);
/// let total = (price + tax)?;
///
/// assert_eq!(total.get_amount(), dec!(11.55));
/// # Ok::<(), FinMoneyError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn zero(currency: FinMoneyCurrency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
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
    pub fn plus_money(&self, other: FinMoney) -> Result<FinMoney, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(FinMoney::new(self.amount + other.amount, self.currency))
    }

    /// Adds a `Decimal` amount to this `FinMoney`.
    pub fn plus_decimal(&self, d: Decimal) -> FinMoney {
        FinMoney::new(self.amount + d, self.currency)
    }

    /// Subtracts another `FinMoney` value from this one, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn minus_money(&self, other: FinMoney) -> Result<FinMoney, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(FinMoney::new(self.amount - other.amount, self.currency))
    }

    /// Subtracts a `Decimal` amount from this `FinMoney`.
    pub fn minus_decimal(&self, d: Decimal) -> FinMoney {
        FinMoney::new(self.amount - d, self.currency)
    }

    /// Multiplies this `FinMoney` by another `FinMoney`, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn multiplied_by_money(&self, other: FinMoney) -> Result<FinMoney, FinMoneyError> {
        self.assert_same_currency(other)?;
        Ok(FinMoney::new(self.amount * other.amount, self.currency))
    }

    /// Multiplies this `FinMoney` by a `Decimal`.
    pub fn multiplied_by_decimal(&self, d: Decimal) -> FinMoney {
        FinMoney::new(self.amount * d, self.currency)
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
        self.compare_to(other)
    }

    /// Compares this `FinMoney` with another, ensuring the same currency.
    ///
    /// # Errors
    ///
    /// Returns `FinMoneyError::CurrencyMismatch` if the currencies don't match.
    pub fn compare_to(&self, other: FinMoney) -> Result<Ordering, FinMoneyError> {
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

    /// Checks if this `FinMoney` is equal to another in amount and currency.
    pub fn is_amount_and_currency_equal_to(&self, other: FinMoney) -> bool {
        self.is_equal_to(other)
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
    pub fn floor(&self) -> FinMoney {
        FinMoney::new(self.amount.floor(), self.currency)
    }

    /// Returns the smallest integer greater than or equal to this `FinMoney`.
    pub fn ceil(&self) -> FinMoney {
        FinMoney::new(self.amount.ceil(), self.currency)
    }

    /// Returns the integer part of this `FinMoney`, removing the fractional part.
    pub fn trunc(&self) -> FinMoney {
        FinMoney::new(self.amount.trunc(), self.currency)
    }

    // -- Properties and Checks --

    /// Checks if the amount is an integer (no fractional part).
    pub fn is_integer(&self) -> bool {
        self.amount.fract().is_zero()
    }

    /// Checks if the amount has a fractional part.
    pub fn has_fraction(&self) -> bool {
        !self.amount.fract().is_zero()
    }

    /// Checks if the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    /// Checks if the amount is positive (greater than zero).
    pub fn is_positive(&self) -> bool {
        self.amount.is_sign_positive() && !self.amount.is_zero()
    }

    /// Checks if the amount is negative (less than zero).
    pub fn is_negative(&self) -> bool {
        self.amount.is_sign_negative() && !self.amount.is_zero()
    }

    /// Checks if the amount is positive or zero.
    pub fn is_positive_or_zero(&self) -> bool {
        self.amount.is_sign_positive()
    }

    /// Checks if the amount is negative or zero.
    pub fn is_negative_or_zero(&self) -> bool {
        self.amount.is_sign_negative() || self.amount.is_zero()
    }

    // -- Utilities --

    /// Returns the square root of the amount.
    ///
    /// # Panics
    ///
    /// Panics if the amount is negative (square root of negative number).
    pub fn sqrt(&self) -> FinMoney {
        FinMoney::new(self.amount.sqrt().unwrap(), self.currency)
    }

    /// Returns the absolute value of the amount.
    pub fn abs(&self) -> FinMoney {
        FinMoney::new(self.amount.abs(), self.currency)
    }

    /// Returns the negated value of the amount.
    pub fn negated(&self) -> FinMoney {
        FinMoney::new(-self.amount, self.currency)
    }

    /// Returns a normalized version of the amount.
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
    pub fn negative_percent_change_from(&self, initial: FinMoney) -> Result<Decimal, FinMoneyError> {
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
    pub fn percent_change(initial: FinMoney, new_value: FinMoney) -> Result<Decimal, FinMoneyError> {
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
    pub fn tick_power10_dp(tick: Decimal) -> Option<u32> {
        // If tick is exactly 10^-dp, then its scale is dp and its coefficient is 1.
        // This avoids powi/multiply allocations and is significantly cheaper.
        let dp = tick.scale();
        if tick == Decimal::new(1, dp) {
            Some(dp)
        } else {
            None
        }
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
    type Output = FinMoney;

    fn mul(self, rhs: Decimal) -> Self::Output {
        self.multiplied_by_decimal(rhs)
    }
}

impl AddAssign for FinMoney {
    fn add_assign(&mut self, rhs: Self) {
        *self = self
            .plus_money(rhs)
            .expect("Currency mismatch in AddAssign");
    }
}

impl SubAssign for FinMoney {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self
            .minus_money(rhs)
            .expect("Currency mismatch in SubAssign");
    }
}

impl fmt::Display for FinMoney {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency.get_code())
    }
}
