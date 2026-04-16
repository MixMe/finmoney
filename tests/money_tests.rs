//! Comprehensive tests for the FinMoney type.

use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyError, FinMoneyRoundingStrategy};
use rust_decimal_macros::dec;

#[test]
fn test_fin_money_creation() {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.50), usd);

    assert_eq!(fin_money.get_amount(), dec!(10.50));
    assert_eq!(fin_money.get_currency_code(), "USD");
    assert_eq!(fin_money.get_precision(), 2);
}

#[test]
fn test_fin_money_zero() {
    let usd = FinMoneyCurrency::USD;
    let zero = FinMoney::zero(usd);

    assert!(zero.is_zero());
    assert_eq!(zero.get_amount(), dec!(0));
}

#[test]
fn test_fin_money_arithmetic() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let fin_money1 = FinMoney::new(dec!(10.50), usd);
    let fin_money2 = FinMoney::new(dec!(5.25), usd);

    // Addition
    let sum = (fin_money1 + fin_money2)?;
    assert_eq!(sum.get_amount(), dec!(15.75));

    // Subtraction
    let diff = (fin_money1 - fin_money2)?;
    assert_eq!(diff.get_amount(), dec!(5.25));

    // Multiplication with decimal
    let product = fin_money1 * dec!(2);
    assert_eq!(product.get_amount(), dec!(21.00));

    // Division
    let quotient =
        fin_money1.divided_by_decimal(dec!(2), FinMoneyRoundingStrategy::MidpointNearestEven)?;
    assert_eq!(quotient.get_amount(), dec!(5.25));

    Ok(())
}

#[test]
fn test_currency_mismatch() {
    let usd = FinMoneyCurrency::USD;
    let eur = FinMoneyCurrency::EUR;
    let usd_fin_money = FinMoney::new(dec!(10), usd);
    let eur_fin_money = FinMoney::new(dec!(10), eur);

    let result = usd_fin_money + eur_fin_money;
    assert!(matches!(
        result,
        Err(FinMoneyError::CurrencyMismatch { .. })
    ));
}

#[test]
fn test_division_by_zero() {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10), usd);

    let result =
        fin_money.divided_by_decimal(dec!(0), FinMoneyRoundingStrategy::MidpointNearestEven);
    assert!(matches!(result, Err(FinMoneyError::DivisionByZero)));
}

#[test]
fn test_comparisons() {
    let usd = FinMoneyCurrency::USD;
    let fin_money1 = FinMoney::new(dec!(10.50), usd);
    let fin_money2 = FinMoney::new(dec!(5.25), usd);

    assert!(fin_money1.is_greater_than(fin_money2));
    assert!(fin_money2.is_less_than(fin_money1));
    assert!(fin_money1.is_greater_than_or_equal(fin_money1));
    assert!(fin_money2.is_less_than_or_equal(fin_money1));

    let min = fin_money1.min(fin_money2);
    let max = fin_money1.max(fin_money2);

    assert_eq!(min.get_amount(), dec!(5.25));
    assert_eq!(max.get_amount(), dec!(10.50));

    // PartialOrd / Ord operators
    assert!(fin_money1 > fin_money2);
    assert!(fin_money2 < fin_money1);
    assert!(fin_money1 >= fin_money1);
    assert!(fin_money2 <= fin_money1);
}

#[test]
fn test_properties() {
    let usd = FinMoneyCurrency::USD;

    let zero = FinMoney::new(dec!(0), usd);
    assert!(zero.is_zero());
    assert!(!zero.is_positive());
    assert!(!zero.is_negative());
    assert!(zero.is_positive_or_zero());
    assert!(zero.is_negative_or_zero());

    let positive = FinMoney::new(dec!(10.50), usd);
    assert!(positive.is_positive());
    assert!(!positive.is_negative());
    assert!(positive.is_positive_or_zero());
    assert!(!positive.is_negative_or_zero());

    let negative = FinMoney::new(dec!(-10.50), usd);
    assert!(!negative.is_positive());
    assert!(negative.is_negative());
    assert!(!negative.is_positive_or_zero());
    assert!(negative.is_negative_or_zero());

    let integer = FinMoney::new(dec!(10), usd);
    assert!(integer.is_integer());
    assert!(!integer.has_fraction());

    let fractional = FinMoney::new(dec!(10.50), usd);
    assert!(!fractional.is_integer());
    assert!(fractional.has_fraction());
}

#[test]
fn test_mathematical_operations() {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(-15.75), usd);

    assert_eq!(fin_money.abs().get_amount(), dec!(15.75));
    assert_eq!(fin_money.negated().get_amount(), dec!(15.75));
    assert_eq!(fin_money.floor().get_amount(), dec!(-16));
    assert_eq!(fin_money.ceil().get_amount(), dec!(-15));
    assert_eq!(fin_money.trunc().get_amount(), dec!(-15));
}

#[test]
fn test_percentage_calculations() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let initial = FinMoney::new(dec!(100), usd);
    let current = FinMoney::new(dec!(110), usd);

    let change = current.percent_change_from(initial)?;
    assert_eq!(change, dec!(10));

    let negative_change = current.negative_percent_change_from(initial)?;
    assert_eq!(negative_change, dec!(-10));

    // Test static methods
    let change2 = FinMoney::percent_change(initial, current)?;
    assert_eq!(change2, dec!(10));

    Ok(())
}

#[test]
fn test_rounding() {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.555), usd);

    let rounded_even =
        fin_money.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointNearestEven);
    assert_eq!(rounded_even.get_amount(), dec!(10.56));

    let rounded_away =
        fin_money.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointAwayFromZero);
    assert_eq!(rounded_away.get_amount(), dec!(10.56));

    let rounded_toward =
        fin_money.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointTowardZero);
    assert_eq!(rounded_toward.get_amount(), dec!(10.55));
}

#[test]
fn test_display() {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.50), usd);

    assert_eq!(format!("{}", fin_money), "10.50 USD");
}

#[test]
fn test_precision_with_creation() {
    let usd = FinMoneyCurrency::USD; // 2 decimal places
    let fin_money = FinMoney::new_with_precision(
        dec!(10.567),
        usd,
        FinMoneyRoundingStrategy::MidpointNearestEven,
    );

    assert_eq!(fin_money.get_amount(), dec!(10.57));
}

#[test]
fn test_rescale() {
    let usd = FinMoneyCurrency::USD; // 2 decimal places
    let fin_money = FinMoney::new(dec!(10.567), usd);

    let rescaled = fin_money.rescale(3);
    assert_eq!(rescaled.get_precision(), 3);
    assert_eq!(rescaled.get_amount(), dec!(10.567));
}

// ============================================================
// Overflow / boundary tests (Requirement 8.1)
// ============================================================

#[test]
fn test_addition_overflow_with_decimal_max() {
    let usd = FinMoneyCurrency::USD;
    let max_money = FinMoney::new(rust_decimal::Decimal::MAX, usd);
    let one = FinMoney::new(dec!(1), usd);

    let result = max_money + one;
    assert!(result.is_err());
    assert!(matches!(result, Err(FinMoneyError::ArithmeticOverflow)));
}

#[test]
fn test_subtraction_overflow_with_decimal_min() {
    let usd = FinMoneyCurrency::USD;
    let min_money = FinMoney::new(rust_decimal::Decimal::MIN, usd);
    let one = FinMoney::new(dec!(1), usd);

    let result = min_money - one;
    assert!(result.is_err());
    assert!(matches!(result, Err(FinMoneyError::ArithmeticOverflow)));
}

#[test]
#[should_panic(expected = "arithmetic overflow")]
fn test_multiplication_overflow_with_decimal_max() {
    let usd = FinMoneyCurrency::USD;
    let max_money = FinMoney::new(rust_decimal::Decimal::MAX, usd);

    let _ = max_money * dec!(2);
}

#[test]
fn test_multiplication_overflow_checked_returns_err() {
    let usd = FinMoneyCurrency::USD;
    let max_money = FinMoney::new(rust_decimal::Decimal::MAX, usd);

    let result = max_money.multiplied_by_decimal(dec!(2));
    assert!(result.is_err());
    assert!(matches!(result, Err(FinMoneyError::ArithmeticOverflow)));
}

#[test]
#[should_panic(expected = "arithmetic overflow")]
fn test_plus_decimal_overflow() {
    let usd = FinMoneyCurrency::USD;
    let max_money = FinMoney::new(rust_decimal::Decimal::MAX, usd);

    let _ = max_money.plus_decimal(dec!(1));
}

#[test]
#[should_panic(expected = "arithmetic overflow")]
fn test_minus_decimal_overflow() {
    let usd = FinMoneyCurrency::USD;
    let min_money = FinMoney::new(rust_decimal::Decimal::MIN, usd);

    let _ = min_money.minus_decimal(dec!(1));
}

#[test]
fn test_multiplied_by_money_overflow() {
    let usd = FinMoneyCurrency::USD;
    let max_money = FinMoney::new(rust_decimal::Decimal::MAX, usd);
    let two = FinMoney::new(dec!(2), usd);

    let result = max_money.multiplied_by_money(two);
    assert!(result.is_err());
    assert!(matches!(result, Err(FinMoneyError::ArithmeticOverflow)));
}

// ============================================================
// Negative value arithmetic tests (Requirement 8.2)
// ============================================================

#[test]
fn test_negative_addition() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(-10.50), usd);
    let b = FinMoney::new(dec!(-5.25), usd);

    let sum = (a + b)?;
    assert_eq!(sum.get_amount(), dec!(-15.75));
    Ok(())
}

#[test]
fn test_negative_subtraction() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(-10.50), usd);
    let b = FinMoney::new(dec!(-5.25), usd);

    let diff = (a - b)?;
    assert_eq!(diff.get_amount(), dec!(-5.25));
    Ok(())
}

#[test]
fn test_negative_multiplication() {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(-10.50), usd);

    // negative * positive = negative
    let result = a * dec!(2);
    assert_eq!(result.get_amount(), dec!(-21.00));

    // negative * negative = positive
    let result2 = a * dec!(-3);
    assert_eq!(result2.get_amount(), dec!(31.50));
}

#[test]
fn test_negative_plus_decimal() {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(-10), usd);

    let result = a.plus_decimal(dec!(-5));
    assert_eq!(result.get_amount(), dec!(-15));

    let result2 = a.plus_decimal(dec!(15));
    assert_eq!(result2.get_amount(), dec!(5));
}

#[test]
fn test_negative_minus_decimal() {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(-10), usd);

    let result = a.minus_decimal(dec!(5));
    assert_eq!(result.get_amount(), dec!(-15));

    let result2 = a.minus_decimal(dec!(-20));
    assert_eq!(result2.get_amount(), dec!(10));
}

#[test]
fn test_negative_multiplied_by_money() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(-5), usd);
    let b = FinMoney::new(dec!(-4), usd);

    let result = a.multiplied_by_money(b)?;
    assert_eq!(result.get_amount(), dec!(20));
    Ok(())
}

// ============================================================
// Precision 0 (JPY) tests (Requirement 8.3)
// ============================================================

#[test]
fn test_jpy_precision_zero_arithmetic() -> Result<(), FinMoneyError> {
    let jpy = FinMoneyCurrency::JPY;
    let a = FinMoney::new(dec!(1000), jpy);
    let b = FinMoney::new(dec!(500), jpy);

    let sum = (a + b)?;
    assert_eq!(sum.get_amount(), dec!(1500));

    let diff = (a - b)?;
    assert_eq!(diff.get_amount(), dec!(500));

    let product = a * dec!(3);
    assert_eq!(product.get_amount(), dec!(3000));
    Ok(())
}

#[test]
fn test_jpy_precision_zero_rounding() {
    let jpy = FinMoneyCurrency::JPY;
    let m = FinMoney::new(dec!(1234.56), jpy);

    let rounded = m.round_dp_with_strategy(0, FinMoneyRoundingStrategy::MidpointNearestEven);
    assert_eq!(rounded.get_amount(), dec!(1235));
}

#[test]
fn test_jpy_precision_zero_allocate() -> Result<(), FinMoneyError> {
    let jpy = FinMoneyCurrency::JPY;
    let total = FinMoney::new(dec!(1000), jpy);

    let parts = total.allocate(&[dec!(1), dec!(1), dec!(1)])?;
    assert_eq!(parts.len(), 3);
    let sum: rust_decimal::Decimal = parts.iter().map(|p| p.get_amount()).sum();
    assert_eq!(sum, dec!(1000));
    Ok(())
}

#[test]
fn test_jpy_format_padded() {
    let jpy = FinMoneyCurrency::JPY;
    let m = FinMoney::new(dec!(12345), jpy);

    assert_eq!(m.format_padded(0), "12345 JPY");
}

// ============================================================
// Precision 28 tests (Requirement 8.4)
// ============================================================

#[test]
fn test_precision_28_arithmetic() -> Result<(), FinMoneyError> {
    let high_prec = FinMoneyCurrency::new(100, "HP28", None::<String>, 28)?;
    let a = FinMoney::new(dec!(1.0000000000000000000000000001), high_prec);
    let b = FinMoney::new(dec!(2.0000000000000000000000000002), high_prec);

    let sum = (a + b)?;
    assert_eq!(sum.get_amount(), dec!(3.0000000000000000000000000003));
    Ok(())
}

#[test]
fn test_precision_28_rounding() -> Result<(), FinMoneyError> {
    let high_prec = FinMoneyCurrency::new(100, "HP28", None::<String>, 28)?;
    let m = FinMoney::new(dec!(1.1234567890123456789012345678), high_prec);

    let rounded = m.round_dp_with_strategy(28, FinMoneyRoundingStrategy::MidpointNearestEven);
    // Should keep all 28 decimal places
    assert_eq!(rounded.get_amount(), dec!(1.1234567890123456789012345678));
    Ok(())
}

#[test]
fn test_precision_28_format_padded() -> Result<(), FinMoneyError> {
    let high_prec = FinMoneyCurrency::new(100, "HP28", None::<String>, 28)?;
    let m = FinMoney::new(dec!(1.5), high_prec);

    let formatted = m.format_padded(28);
    assert!(formatted.ends_with(" HP28"));
    // Should have 28 digits after decimal point
    let amount_part = formatted.strip_suffix(" HP28").unwrap();
    let frac = amount_part.split('.').nth(1).unwrap();
    assert_eq!(frac.len(), 28);
    Ok(())
}

// ============================================================
// AddAssign / SubAssign tests
// ============================================================

#[test]
fn test_add_assign_same_currency() {
    let usd = FinMoneyCurrency::USD;
    let mut a = FinMoney::new(dec!(10), usd);
    let b = FinMoney::new(dec!(5.50), usd);

    a += b;
    assert_eq!(a.get_amount(), dec!(15.50));
}

#[test]
fn test_sub_assign_same_currency() {
    let usd = FinMoneyCurrency::USD;
    let mut a = FinMoney::new(dec!(20), usd);
    let b = FinMoney::new(dec!(7.25), usd);

    a -= b;
    assert_eq!(a.get_amount(), dec!(12.75));
}

#[test]
#[should_panic(expected = "currency mismatch")]
fn test_add_assign_currency_mismatch_panics() {
    let mut a = FinMoney::new(dec!(10), FinMoneyCurrency::USD);
    let b = FinMoney::new(dec!(5), FinMoneyCurrency::EUR);

    a += b;
}

#[test]
#[should_panic(expected = "currency mismatch")]
fn test_sub_assign_currency_mismatch_panics() {
    let mut a = FinMoney::new(dec!(10), FinMoneyCurrency::USD);
    let b = FinMoney::new(dec!(5), FinMoneyCurrency::EUR);

    a -= b;
}

#[test]
fn test_add_assign_accumulate_loop() {
    let usd = FinMoneyCurrency::USD;
    let mut total = FinMoney::zero(usd);

    for i in 1..=10 {
        total += FinMoney::new(rust_decimal::Decimal::from(i), usd);
    }
    // 1+2+...+10 = 55
    assert_eq!(total.get_amount(), dec!(55));
}

// ============================================================
// Unchecked arithmetic tests
// ============================================================

#[test]
fn test_unchecked_plus() {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(10), usd);
    let b = FinMoney::new(dec!(20), usd);

    let result = a.unchecked_plus(b);
    assert_eq!(result.get_amount(), dec!(30));
}

#[test]
fn test_unchecked_minus() {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(30), usd);
    let b = FinMoney::new(dec!(10), usd);

    let result = a.unchecked_minus(b);
    assert_eq!(result.get_amount(), dec!(20));
}

#[test]
fn test_unchecked_mul() {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(10.50), usd);

    let result = a.unchecked_mul(dec!(3));
    assert_eq!(result.get_amount(), dec!(31.50));
}

#[test]
#[should_panic(expected = "unchecked_plus")]
fn test_unchecked_plus_currency_mismatch_panics() {
    let a = FinMoney::new(dec!(10), FinMoneyCurrency::USD);
    let b = FinMoney::new(dec!(5), FinMoneyCurrency::EUR);

    a.unchecked_plus(b);
}

#[test]
#[should_panic(expected = "unchecked_minus")]
fn test_unchecked_minus_currency_mismatch_panics() {
    let a = FinMoney::new(dec!(10), FinMoneyCurrency::USD);
    let b = FinMoney::new(dec!(5), FinMoneyCurrency::EUR);

    a.unchecked_minus(b);
}

// ============================================================
// Tick normalize fix tests (trailing zeros)
// ============================================================

#[test]
fn test_to_tick_with_trailing_zeros() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let price = FinMoney::new(dec!(10.567), usd);

    // 0.0001 with trailing zeros (simulates exchange data with extra scale)
    let tick_with_trailing = rust_decimal::Decimal::new(100000, 9); // 0.000100000

    let rounded = price.to_tick_nearest(tick_with_trailing)?;
    // Should use fast path after normalize, same as dec!(0.0001)
    let rounded_clean = price.to_tick_nearest(dec!(0.0001))?;

    assert_eq!(rounded.get_amount(), rounded_clean.get_amount());
    Ok(())
}

#[test]
fn test_is_multiple_of_tick_with_trailing_zeros() {
    let usd = FinMoneyCurrency::USD;
    let price = FinMoney::new(dec!(10.50), usd);

    // 0.01 with trailing zeros
    let tick_with_trailing = rust_decimal::Decimal::new(1000000, 8); // 0.01000000

    assert!(price.is_multiple_of_tick(tick_with_trailing));
    assert!(price.is_multiple_of_tick(dec!(0.01)));
}

#[test]
fn test_to_tick_trailing_zeros_power_of_ten_fast_path() -> Result<(), FinMoneyError> {
    let btc = FinMoneyCurrency::BTC;
    let amount = FinMoney::new(dec!(0.12345678), btc);

    // 0.00010000 — should normalize to 0.0001 and use fast path (dp=4)
    let tick = rust_decimal::Decimal::new(10000, 8); // 0.00010000
    let rounded = amount.to_tick_down(tick)?;

    assert_eq!(rounded.get_amount(), dec!(0.1234));
    Ok(())
}
