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
    let product = (fin_money1 * dec!(2))?;
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
fn test_comparisons() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let fin_money1 = FinMoney::new(dec!(10.50), usd);
    let fin_money2 = FinMoney::new(dec!(5.25), usd);

    assert!(fin_money1.is_greater_than(fin_money2)?);
    assert!(fin_money2.is_less_than(fin_money1)?);
    assert!(fin_money1.is_greater_than_or_equal(fin_money1)?);
    assert!(fin_money2.is_less_than_or_equal(fin_money1)?);

    let min: FinMoney = fin_money1.min(fin_money2)?;
    let max: FinMoney = fin_money1.max(fin_money2)?;

    assert_eq!(min.get_amount(), dec!(5.25));
    assert_eq!(max.get_amount(), dec!(10.50));

    Ok(())
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
fn test_rescale() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD; // 2 decimal places
    let fin_money = FinMoney::new(dec!(10.567), usd);

    let rescaled = fin_money.rescale(3)?;
    assert_eq!(rescaled.get_precision(), 3);
    assert_eq!(rescaled.get_amount(), dec!(10.567));

    Ok(())
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
fn test_multiplication_overflow_with_decimal_max() {
    let usd = FinMoneyCurrency::USD;
    let max_money = FinMoney::new(rust_decimal::Decimal::MAX, usd);

    let result = max_money * dec!(2);
    assert!(result.is_err());
    assert!(matches!(result, Err(FinMoneyError::ArithmeticOverflow)));
}

#[test]
fn test_plus_decimal_overflow() {
    let usd = FinMoneyCurrency::USD;
    let max_money = FinMoney::new(rust_decimal::Decimal::MAX, usd);

    let result = max_money.plus_decimal(dec!(1));
    assert!(result.is_err());
    assert!(matches!(result, Err(FinMoneyError::ArithmeticOverflow)));
}

#[test]
fn test_minus_decimal_overflow() {
    let usd = FinMoneyCurrency::USD;
    let min_money = FinMoney::new(rust_decimal::Decimal::MIN, usd);

    let result = min_money.minus_decimal(dec!(1));
    assert!(result.is_err());
    assert!(matches!(result, Err(FinMoneyError::ArithmeticOverflow)));
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
fn test_negative_multiplication() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(-10.50), usd);

    // negative * positive = negative
    let result = (a * dec!(2))?;
    assert_eq!(result.get_amount(), dec!(-21.00));

    // negative * negative = positive
    let result2 = (a * dec!(-3))?;
    assert_eq!(result2.get_amount(), dec!(31.50));
    Ok(())
}

#[test]
fn test_negative_plus_decimal() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(-10), usd);

    let result = a.plus_decimal(dec!(-5))?;
    assert_eq!(result.get_amount(), dec!(-15));

    let result2 = a.plus_decimal(dec!(15))?;
    assert_eq!(result2.get_amount(), dec!(5));
    Ok(())
}

#[test]
fn test_negative_minus_decimal() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let a = FinMoney::new(dec!(-10), usd);

    let result = a.minus_decimal(dec!(5))?;
    assert_eq!(result.get_amount(), dec!(-15));

    let result2 = a.minus_decimal(dec!(-20))?;
    assert_eq!(result2.get_amount(), dec!(10));
    Ok(())
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

    let product = (a * dec!(3))?;
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
