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
    assert!(matches!(result, Err(FinMoneyError::CurrencyMismatch { .. })));
}

#[test]
fn test_division_by_zero() {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10), usd);

    let result = fin_money.divided_by_decimal(dec!(0), FinMoneyRoundingStrategy::MidpointNearestEven);
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

    let rounded_even = fin_money.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointNearestEven);
    assert_eq!(rounded_even.get_amount(), dec!(10.56));

    let rounded_away = fin_money.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointAwayFromZero);
    assert_eq!(rounded_away.get_amount(), dec!(10.56));

    let rounded_toward = fin_money.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointTowardZero);
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
