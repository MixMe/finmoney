//! Tests for currency functionality.

use finmoney::{FinMoneyCurrency, FinMoneyError};

#[test]
fn test_currency_creation() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::new(1, "USD", Some("US Dollar"), 2)?;

    assert_eq!(usd.get_id(), 1);
    assert_eq!(usd.get_code(), "USD");
    assert_eq!(usd.get_name(), Some("US Dollar"));
    assert_eq!(usd.get_precision(), 2);

    Ok(())
}

#[test]
fn test_currency_creation_without_name() -> Result<(), FinMoneyError> {
    let btc = FinMoneyCurrency::new(2, "BTC", None::<String>, 8)?;

    assert_eq!(btc.get_id(), 2);
    assert_eq!(btc.get_code(), "BTC");
    assert_eq!(btc.get_name(), None);
    assert_eq!(btc.get_precision(), 8);

    Ok(())
}

#[test]
fn test_currency_invalid_precision() {
    let result = FinMoneyCurrency::new(1, "USD", None::<String>, 29);
    assert!(matches!(result, Err(FinMoneyError::InvalidPrecision(29))));
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
fn test_currency_with_precision() -> Result<(), FinMoneyError> {
    let usd = FinMoneyCurrency::USD;
    let usd_high_precision = usd.with_precision(4)?;

    assert_eq!(usd_high_precision.get_id(), usd.get_id());
    assert_eq!(usd_high_precision.get_code(), usd.get_code());
    assert_eq!(usd_high_precision.get_precision(), 4);

    // Test invalid precision
    let result = usd.with_precision(29);
    assert!(matches!(result, Err(FinMoneyError::InvalidPrecision(29))));

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

#[test]
fn test_currency_new_from_tiny() -> Result<(), FinMoneyError> {
    use tinystr::TinyAsciiStr;

    // Test with both code and name
    let code: TinyAsciiStr<16> = "USD".parse().unwrap();
    let name: TinyAsciiStr<52> = "US Dollar".parse().unwrap();
    let usd = FinMoneyCurrency::new_from_tiny(1, code, Some(name), 2)?;

    assert_eq!(usd.get_id(), 1);
    assert_eq!(usd.get_code(), "USD");
    assert_eq!(usd.get_name(), Some("US Dollar"));
    assert_eq!(usd.get_precision(), 2);

    // Test with code only (no name)
    let btc_code: TinyAsciiStr<16> = "BTC".parse().unwrap();
    let btc = FinMoneyCurrency::new_from_tiny(2, btc_code, None, 8)?;

    assert_eq!(btc.get_id(), 2);
    assert_eq!(btc.get_code(), "BTC");
    assert_eq!(btc.get_name(), None);
    assert_eq!(btc.get_precision(), 8);

    Ok(())
}

#[test]
fn test_currency_new_from_tiny_invalid_precision() {
    use tinystr::TinyAsciiStr;

    let code: TinyAsciiStr<16> = "USD".parse().unwrap();
    let result = FinMoneyCurrency::new_from_tiny(1, code, None, 29);

    assert!(matches!(result, Err(FinMoneyError::InvalidPrecision(29))));
}

#[test]
fn test_currency_new_from_tiny_performance() -> Result<(), FinMoneyError> {
    use tinystr::TinyAsciiStr;

    // Pre-calculate the TinyAsciiStr values
    let code: TinyAsciiStr<16> = "PERF".parse().unwrap();
    let name: TinyAsciiStr<52> = "Performance Test Currency".parse().unwrap();

    // This should be more efficient than using new() with String conversion
    let currency = FinMoneyCurrency::new_from_tiny(999, code, Some(name), 4)?;

    assert_eq!(currency.get_id(), 999);
    assert_eq!(currency.get_code(), "PERF");
    assert_eq!(currency.get_name(), Some("Performance Test Currency"));
    assert_eq!(currency.get_precision(), 4);

    Ok(())
}

// ============================================================
// New currency constants — precision checks (Requirements 6.1, 6.2)
// ============================================================

#[test]
fn test_gbp_constant() {
    let gbp = FinMoneyCurrency::GBP;
    assert_eq!(gbp.get_id(), 5);
    assert_eq!(gbp.get_code(), "GBP");
    assert_eq!(gbp.get_precision(), 2);
}

#[test]
fn test_jpy_constant() {
    let jpy = FinMoneyCurrency::JPY;
    assert_eq!(jpy.get_id(), 6);
    assert_eq!(jpy.get_code(), "JPY");
    assert_eq!(jpy.get_precision(), 0);
}

#[test]
fn test_chf_constant() {
    let chf = FinMoneyCurrency::CHF;
    assert_eq!(chf.get_id(), 7);
    assert_eq!(chf.get_code(), "CHF");
    assert_eq!(chf.get_precision(), 2);
}

#[test]
fn test_cny_constant() {
    let cny = FinMoneyCurrency::CNY;
    assert_eq!(cny.get_id(), 8);
    assert_eq!(cny.get_code(), "CNY");
    assert_eq!(cny.get_precision(), 2);
}

#[test]
fn test_rub_constant() {
    let rub = FinMoneyCurrency::RUB;
    assert_eq!(rub.get_id(), 9);
    assert_eq!(rub.get_code(), "RUB");
    assert_eq!(rub.get_precision(), 2);
}

#[test]
fn test_usdt_constant() {
    let usdt = FinMoneyCurrency::USDT;
    assert_eq!(usdt.get_id(), 10);
    assert_eq!(usdt.get_code(), "USDT");
    assert_eq!(usdt.get_precision(), 6);
}

#[test]
fn test_sol_constant() {
    let sol = FinMoneyCurrency::SOL;
    assert_eq!(sol.get_id(), 11);
    assert_eq!(sol.get_code(), "SOL");
    assert_eq!(sol.get_precision(), 9);
}

// ============================================================
// all_predefined() tests (Requirement 6.3)
// ============================================================

#[test]
fn test_all_predefined_count() {
    let all = FinMoneyCurrency::all_predefined();
    assert_eq!(all.len(), 11);
}

#[test]
fn test_all_predefined_contains_all_currencies() {
    let all = FinMoneyCurrency::all_predefined();

    let expected = [
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

    for (i, currency) in expected.iter().enumerate() {
        assert_eq!(&all[i], currency, "Mismatch at index {}", i);
    }
}

#[test]
fn test_all_predefined_unique_ids() {
    let all = FinMoneyCurrency::all_predefined();
    let mut ids: Vec<i32> = all.iter().map(|c| c.get_id()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(
        ids.len(),
        11,
        "All predefined currencies must have unique IDs"
    );
}
