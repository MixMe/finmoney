//! Tests for tick handling functionality.

use finmoney::{FinMoney, FinMoneyCurrency, MoneyError, MoneyRoundingStrategy};
use rust_decimal_macros::dec;

#[test]
fn test_tick_rounding_basic() -> Result<(), MoneyError> {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.567), usd);

    // Round to 0.25 tick
    let rounded = fin_money.to_tick_nearest(dec!(0.25))?;
    assert_eq!(rounded.get_amount(), dec!(10.50));

    // Round to 0.10 tick
    let rounded = fin_money.to_tick_nearest(dec!(0.10))?;
    assert_eq!(rounded.get_amount(), dec!(10.60));

    // Round to 0.01 tick
    let rounded = fin_money.to_tick_nearest(dec!(0.01))?;
    assert_eq!(rounded.get_amount(), dec!(10.57));

    Ok(())
}

#[test]
fn test_tick_rounding_directional() -> Result<(), MoneyError> {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.567), usd);

    let tick_down = fin_money.to_tick_down(dec!(0.25))?;
    assert_eq!(tick_down.get_amount(), dec!(10.50));

    let tick_up = fin_money.to_tick_up(dec!(0.25))?;
    assert_eq!(tick_up.get_amount(), dec!(10.75));

    let tick_nearest = fin_money.to_tick_nearest(dec!(0.25))?;
    assert_eq!(tick_nearest.get_amount(), dec!(10.50));

    Ok(())
}

#[test]
fn test_tick_validation() {
    let usd = FinMoneyCurrency::USD;

    let valid_price = FinMoney::new(dec!(10.50), usd);
    assert!(valid_price.is_multiple_of_tick(dec!(0.25)));
    assert!(valid_price.is_multiple_of_tick(dec!(0.50)));
    assert!(valid_price.is_multiple_of_tick(dec!(0.01)));

    let invalid_price = FinMoney::new(dec!(10.567), usd);
    assert!(!invalid_price.is_multiple_of_tick(dec!(0.25)));
    assert!(!invalid_price.is_multiple_of_tick(dec!(0.10)));
    assert!(!invalid_price.is_multiple_of_tick(dec!(0.01)));
}

#[test]
fn test_tick_power_of_ten() -> Result<(), MoneyError> {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.567), usd);

    // Test power-of-ten ticks (should use fast path)
    let rounded_001 = fin_money.to_tick_nearest(dec!(0.001))?;
    assert_eq!(rounded_001.get_amount(), dec!(10.567));

    let rounded_01 = fin_money.to_tick_nearest(dec!(0.01))?;
    assert_eq!(rounded_01.get_amount(), dec!(10.57));

    let rounded_1 = fin_money.to_tick_nearest(dec!(1))?;
    assert_eq!(rounded_1.get_amount(), dec!(11));

    Ok(())
}

#[test]
fn test_tick_power10_dp_helper() {
    assert_eq!(FinMoney::tick_power10_dp(dec!(0.001)), Some(3));
    assert_eq!(FinMoney::tick_power10_dp(dec!(0.01)), Some(2));
    assert_eq!(FinMoney::tick_power10_dp(dec!(0.1)), Some(1));
    assert_eq!(FinMoney::tick_power10_dp(dec!(1)), Some(0));

    // Non-power-of-ten ticks
    assert_eq!(FinMoney::tick_power10_dp(dec!(0.25)), None);
    assert_eq!(FinMoney::tick_power10_dp(dec!(0.33)), None);
    assert_eq!(FinMoney::tick_power10_dp(dec!(5)), None);
}

#[test]
fn test_tick_invalid() {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.50), usd);

    // Zero tick should return error
    let result = fin_money.to_tick_nearest(dec!(0));
    assert!(matches!(result, Err(MoneyError::InvalidTick)));

    // Negative tick should return error
    let result = fin_money.to_tick_nearest(dec!(-0.25));
    assert!(matches!(result, Err(MoneyError::InvalidTick)));
}

#[test]
fn test_tick_edge_cases() -> Result<(), MoneyError> {
    let usd = FinMoneyCurrency::USD;

    // Test with zero amount
    let zero = FinMoney::zero(usd);
    let rounded = zero.to_tick_nearest(dec!(0.25))?;
    assert_eq!(rounded.get_amount(), dec!(0));

    // Test with negative amount
    let negative = FinMoney::new(dec!(-10.567), usd);
    let rounded = negative.to_tick_nearest(dec!(0.25))?;
    assert_eq!(rounded.get_amount(), dec!(-10.50));

    // Test with very small tick
    let fin_money = FinMoney::new(dec!(10.123456789), usd);
    let rounded = fin_money.to_tick_nearest(dec!(0.000000001))?;
    assert_eq!(rounded.get_amount(), dec!(10.123456789));

    Ok(())
}

#[test]
fn test_tick_rounding_strategies() -> Result<(), MoneyError> {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.625), usd); // Exactly between 10.50 and 10.75

    let rounded_even = fin_money.to_tick(dec!(0.25), MoneyRoundingStrategy::MidpointNearestEven)?;
    assert_eq!(rounded_even.get_amount(), dec!(10.50)); // Even multiple of 0.25

    let rounded_away = fin_money.to_tick(dec!(0.25), MoneyRoundingStrategy::MidpointAwayFromZero)?;
    assert_eq!(rounded_away.get_amount(), dec!(10.75));

    let rounded_toward = fin_money.to_tick(dec!(0.25), MoneyRoundingStrategy::MidpointTowardZero)?;
    assert_eq!(rounded_toward.get_amount(), dec!(10.50));

    Ok(())
}

#[test]
fn test_tick_large_values() -> Result<(), MoneyError> {
    let usd = FinMoneyCurrency::USD;

    // Test with large tick size
    let fin_money = FinMoney::new(dec!(1234.56), usd);
    let rounded = fin_money.to_tick_nearest(dec!(100))?;
    assert_eq!(rounded.get_amount(), dec!(1200));

    // Test with very large amount
    let large_fin_money = FinMoney::new(dec!(999999.99), usd);
    let rounded = large_fin_money.to_tick_nearest(dec!(0.01))?;
    assert_eq!(rounded.get_amount(), dec!(999999.99));

    Ok(())
}

#[test]
fn test_is_multiple_of_tick_edge_cases() {
    let usd = FinMoneyCurrency::USD;

    // Zero tick should return false
    let fin_money = FinMoney::new(dec!(10.50), usd);
    assert!(!fin_money.is_multiple_of_tick(dec!(0)));

    // Zero amount should be multiple of any positive tick
    let zero = FinMoney::zero(usd);
    assert!(zero.is_multiple_of_tick(dec!(0.25)));
    assert!(zero.is_multiple_of_tick(dec!(1)));
    assert!(zero.is_multiple_of_tick(dec!(100)));
}
