//! Tests for currency functionality.

use finmoney::{FinMoneyCurrency, MoneyError};

#[test]
fn test_currency_creation() -> Result<(), MoneyError> {
    let usd = FinMoneyCurrency::new(1, "USD".to_string(), Some("US Dollar".to_string()), 2)?;

    assert_eq!(usd.get_id(), 1);
    assert_eq!(usd.get_code(), "USD");
    assert_eq!(usd.get_name(), Some("US Dollar"));
    assert_eq!(usd.get_precision(), 2);

    Ok(())
}

#[test]
fn test_currency_creation_without_name() -> Result<(), MoneyError> {
    let btc = FinMoneyCurrency::new(2, "BTC".to_string(), None, 8)?;

    assert_eq!(btc.get_id(), 2);
    assert_eq!(btc.get_code(), "BTC");
    assert_eq!(btc.get_name(), None);
    assert_eq!(btc.get_precision(), 8);

    Ok(())
}

#[test]
fn test_currency_invalid_precision() {
    let result = FinMoneyCurrency::new(1, "USD".to_string(), None, 29);
    assert!(matches!(result, Err(MoneyError::InvalidPrecision(29))));
}

#[test]
fn test_currency_sanitized_creation() {
    // Test with invalid characters that should be sanitized
    let currency =
        FinMoneyCurrency::new_sanitized(1, "US$".to_string(), Some("US Dollar™".to_string()), 2);

    assert_eq!(currency.get_id(), 1);
    assert_eq!(currency.get_code(), "US$"); // Should work as $ is ASCII
    assert_eq!(currency.get_precision(), 2);

    // Test precision clamping
    let currency_high_precision = FinMoneyCurrency::new_sanitized(
        2,
        "BTC".to_string(),
        None,
        50, // Should be clamped to 28
    );

    assert_eq!(currency_high_precision.get_precision(), 28);
}

#[test]
fn test_currency_with_precision() -> Result<(), MoneyError> {
    let usd = FinMoneyCurrency::USD;
    let usd_high_precision = usd.with_precision(4)?;

    assert_eq!(usd_high_precision.get_id(), usd.get_id());
    assert_eq!(usd_high_precision.get_code(), usd.get_code());
    assert_eq!(usd_high_precision.get_precision(), 4);

    // Test invalid precision
    let result = usd.with_precision(29);
    assert!(matches!(result, Err(MoneyError::InvalidPrecision(29))));

    Ok(())
}

#[test]
fn test_currency_comparison() {
    let usd1 = FinMoneyCurrency::USD;
    let usd2 = FinMoneyCurrency::USD;
    let eur = FinMoneyCurrency::EUR;

    assert!(usd1.is_same_currency(&usd2));
    assert!(!usd1.is_same_currency(&eur));
}

#[test]
fn test_predefined_currencies() {
    let usd = FinMoneyCurrency::USD;
    assert_eq!(usd.get_id(), 1);
    assert_eq!(usd.get_code(), "USD");
    assert_eq!(usd.get_precision(), 2);

    let eur = FinMoneyCurrency::EUR;
    assert_eq!(eur.get_id(), 2);
    assert_eq!(eur.get_code(), "EUR");
    assert_eq!(eur.get_precision(), 2);

    let btc = FinMoneyCurrency::BTC;
    assert_eq!(btc.get_id(), 3);
    assert_eq!(btc.get_code(), "BTC");
    assert_eq!(btc.get_precision(), 8);

    let eth = FinMoneyCurrency::ETH;
    assert_eq!(eth.get_id(), 4);
    assert_eq!(eth.get_code(), "ETH");
    assert_eq!(eth.get_precision(), 18);
}

#[test]
fn test_currency_default() {
    let default_currency = FinMoneyCurrency::default();

    assert_eq!(default_currency.get_id(), 0);
    assert_eq!(default_currency.get_code(), "UNDEFINED");
    assert_eq!(default_currency.get_name(), None);
    assert_eq!(default_currency.get_precision(), 8);
}

#[test]
fn test_currency_equality() {
    let usd1 = FinMoneyCurrency::USD;
    let usd2 = FinMoneyCurrency::USD;
    let eur = FinMoneyCurrency::EUR;

    assert_eq!(usd1, usd2);
    assert_ne!(usd1, eur);
}

#[test]
fn test_currency_long_names_and_codes() {
    // Test very long currency code (should be truncated)
    let long_code = "VERYLONGCURRENCYCODE".to_string();
    let currency = FinMoneyCurrency::new_sanitized(1, long_code, None, 2);

    // Should be truncated to 16 characters max
    assert!(currency.get_code().len() <= 16);

    // Test very long currency name (should be truncated)
    let long_name = "Very Long Currency Name That Exceeds The Maximum Length Allowed".to_string();
    let currency = FinMoneyCurrency::new_sanitized(2, "TEST".to_string(), Some(long_name), 2);

    // Should be truncated to 52 characters max
    if let Some(name) = currency.get_name() {
        assert!(name.len() <= 52);
    }
}
