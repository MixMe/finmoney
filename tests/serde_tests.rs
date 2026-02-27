#![cfg(feature = "serde")]

//! Serde round-trip tests for the finmoney library.
//!
//! All tests require the `serde` feature to be enabled.
//! Uses `serde_json` for serialization and `proptest` for property-based tests.

use finmoney::{Decimal, FinMoney, FinMoneyCurrency};
use proptest::prelude::*;
use rust_decimal_macros::dec;

// ---------------------------------------------------------------------------
// Unit Tests
// ---------------------------------------------------------------------------

#[test]
fn unit_roundtrip_finmoney() {
    let money = FinMoney::new(dec!(1234.56), FinMoneyCurrency::USD);
    let json = serde_json::to_string(&money).unwrap();
    let deserialized: FinMoney = serde_json::from_str(&json).unwrap();
    assert_eq!(money, deserialized);
}

#[test]
fn unit_roundtrip_finmoney_currency() {
    let currency = FinMoneyCurrency::USD;
    let json = serde_json::to_string(&currency).unwrap();
    let deserialized: FinMoneyCurrency = serde_json::from_str(&json).unwrap();
    assert_eq!(currency, deserialized);
}

#[test]
fn unit_amount_serialized_as_string() {
    let money = FinMoney::new(dec!(99.99), FinMoneyCurrency::EUR);
    let json = serde_json::to_string(&money).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let amount_value = &parsed["amount"];
    assert!(
        amount_value.is_string(),
        "amount should be a JSON string, got: {}",
        amount_value
    );
}

#[test]
fn unit_invalid_json_returns_error() {
    let bad_json = r#"{"amount": null, "currency": {}}"#;
    let result = serde_json::from_str::<FinMoney>(bad_json);
    assert!(result.is_err(), "Deserializing invalid JSON should fail");

    let garbage = "not json at all";
    let result2 = serde_json::from_str::<FinMoney>(garbage);
    assert!(result2.is_err(), "Deserializing garbage should fail");
}

// ---------------------------------------------------------------------------
// Property-Based Tests
// ---------------------------------------------------------------------------

proptest! {
    // Feature: project-improvements, Property 7: Serde round-trip
    // **Validates: Requirements 7.1, 7.2**
    #[test]
    fn p7_serde_roundtrip_finmoney(
        amount_raw in -1_000_000_000i64..=1_000_000_000i64,
        currency_idx in 0..11usize,
    ) {
        let currency = FinMoneyCurrency::all_predefined()[currency_idx];
        let precision = currency.get_precision() as u32;
        let effective_scale = precision.min(4);
        let amount = Decimal::new(amount_raw, effective_scale);
        let money = FinMoney::new(amount, currency);

        let json = serde_json::to_string(&money).unwrap();
        let deserialized: FinMoney = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(money, deserialized);
    }

    // Feature: project-improvements, Property 7: Serde round-trip (FinMoneyCurrency)
    // **Validates: Requirements 7.1, 7.2**
    #[test]
    fn p7_serde_roundtrip_finmoney_currency(currency_idx in 0..11usize) {
        let currency = FinMoneyCurrency::all_predefined()[currency_idx];
        let json = serde_json::to_string(&currency).unwrap();
        let deserialized: FinMoneyCurrency = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(currency, deserialized);
    }

    // Feature: project-improvements, Property 8: Serde — amount как строка
    // **Validates: Requirements 7.3**
    #[test]
    fn p8_serde_amount_is_string(
        amount_raw in -1_000_000_000i64..=1_000_000_000i64,
        currency_idx in 0..11usize,
    ) {
        let currency = FinMoneyCurrency::all_predefined()[currency_idx];
        let precision = currency.get_precision() as u32;
        let effective_scale = precision.min(4);
        let amount = Decimal::new(amount_raw, effective_scale);
        let money = FinMoney::new(amount, currency);

        let json = serde_json::to_string(&money).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let amount_value = &parsed["amount"];
        prop_assert!(
            amount_value.is_string(),
            "amount should be a JSON string, got: {}",
            amount_value
        );
    }
}
