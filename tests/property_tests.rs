//! Property-based tests for the finmoney library.
//!
//! Uses `proptest` to verify correctness properties across random inputs.
//! Each test is annotated with the property number from the design document.

use finmoney::{Decimal, FinMoney, FinMoneyCurrency, FinMoneyError, FinMoneyRoundingStrategy};
use proptest::prelude::*;

fn test_currencies() -> Vec<FinMoneyCurrency> {
    vec![
        FinMoneyCurrency::new(1, "USD", None::<&str>, 2).unwrap(),
        FinMoneyCurrency::new(2, "EUR", None::<&str>, 2).unwrap(),
        FinMoneyCurrency::new(3, "BTC", None::<&str>, 8).unwrap(),
        FinMoneyCurrency::new(4, "ETH", None::<&str>, 18).unwrap(),
        FinMoneyCurrency::new(5, "GBP", None::<&str>, 2).unwrap(),
        FinMoneyCurrency::new(6, "JPY", None::<&str>, 0).unwrap(),
        FinMoneyCurrency::new(7, "CHF", None::<&str>, 2).unwrap(),
        FinMoneyCurrency::new(8, "CNY", None::<&str>, 2).unwrap(),
        FinMoneyCurrency::new(9, "RUB", None::<&str>, 2).unwrap(),
        FinMoneyCurrency::new(10, "USDT", None::<&str>, 6).unwrap(),
        FinMoneyCurrency::new(11, "SOL", None::<&str>, 9).unwrap(),
    ]
}

// ---------------------------------------------------------------------------
// Generators
// ---------------------------------------------------------------------------

/// Generate a random FinMoneyError variant.
fn arb_finmoney_error() -> impl Strategy<Value = FinMoneyError> {
    prop_oneof![
        (".*", ".*")
            .prop_map(|(expected, actual)| FinMoneyError::CurrencyMismatch { expected, actual }),
        Just(FinMoneyError::DivisionByZero),
        any::<u32>().prop_map(FinMoneyError::InvalidPrecision),
        Just(FinMoneyError::InvalidTick),
        ".*".prop_map(FinMoneyError::InvalidCurrencyCode),
        ".*".prop_map(FinMoneyError::InvalidCurrencyName),
        Just(FinMoneyError::ArithmeticOverflow),
        ".*".prop_map(FinMoneyError::InvalidAmount),
    ]
}

/// Generate a random rounding strategy.
fn arb_rounding_strategy() -> impl Strategy<Value = FinMoneyRoundingStrategy> {
    prop_oneof![
        Just(FinMoneyRoundingStrategy::MidpointNearestEven),
        Just(FinMoneyRoundingStrategy::MidpointAwayFromZero),
        Just(FinMoneyRoundingStrategy::MidpointTowardZero),
        Just(FinMoneyRoundingStrategy::ToZero),
        Just(FinMoneyRoundingStrategy::AwayFromZero),
        Just(FinMoneyRoundingStrategy::ToNegativeInfinity),
        Just(FinMoneyRoundingStrategy::ToPositiveInfinity),
    ]
}

// ---------------------------------------------------------------------------
// Property Tests
// ---------------------------------------------------------------------------

proptest! {
    // Feature: project-improvements, Property 1: Checked arithmetic detects overflow
    // **Validates: Requirements 2.1, 2.2, 2.3**
    #[test]
    fn p1_checked_overflow_add(currency_idx in 0..11usize) {
        let currency = test_currencies()[currency_idx];
        let a = FinMoney::new(Decimal::MAX, currency);
        let b = FinMoney::new(Decimal::from(1), currency);
        let result = a.plus_money(b);
        prop_assert!(result.is_err());
        prop_assert!(result.unwrap_err().is_overflow());
    }

    #[test]
    fn p1_checked_overflow_sub(currency_idx in 0..11usize) {
        let currency = test_currencies()[currency_idx];
        let a = FinMoney::new(Decimal::MIN, currency);
        let b = FinMoney::new(Decimal::from(1), currency);
        let result = a.minus_money(b);
        prop_assert!(result.is_err());
        prop_assert!(result.unwrap_err().is_overflow());
    }

    #[test]
    fn p1_checked_overflow_mul(currency_idx in 0..11usize) {
        let currency = test_currencies()[currency_idx];
        let a = FinMoney::new(Decimal::MAX, currency);
        let result = a.multiplied_by_decimal(Decimal::from(2));
        prop_assert!(result.is_err());
        prop_assert!(result.unwrap_err().is_overflow());
    }

    // Feature: project-improvements, Property 2: Display formatting of FinMoneyCurrency
    // **Validates: Requirements 3.1, 3.2, 3.3**
    #[test]
    fn p2_display_predefined(currency_idx in 0..11usize) {
        let currency = test_currencies()[currency_idx];
        let display = format!("{}", currency);
        let code = currency.get_code();
        // All predefined currencies have no name set, so Display should be just the code
        prop_assert!(display.contains(code), "Display '{}' must contain code '{}'", display, code);
        if currency.get_name().is_some() {
            let name = currency.get_name().unwrap();
            let expected = format!("{} ({})", code, name);
            prop_assert_eq!(display, expected);
        } else {
            prop_assert_eq!(display, code);
        }
    }

    #[test]
    fn p2_display_with_name(
        code in "[A-Z]{3,5}",
        name in "[A-Za-z ]{3,20}",
    ) {
        if let Ok(currency) = FinMoneyCurrency::new(100, &code, Some(&name), 2) {
            let display = format!("{}", currency);
            prop_assert!(display.contains(currency.get_code()));
            if let Some(n) = currency.get_name() {
                let expected = format!("{} ({})", currency.get_code(), n);
                prop_assert_eq!(display, expected);
            }
        }
    }

    #[test]
    fn p2_display_without_name(
        code in "[A-Z]{3,5}",
    ) {
        if let Ok(currency) = FinMoneyCurrency::new(100, &code, None::<&str>, 2) {
            let display = format!("{}", currency);
            prop_assert_eq!(display, currency.get_code());
        }
    }

    // Feature: project-improvements, Property 3: from_i64 correctness
    // **Validates: Requirements 4.1**
    #[test]
    fn p3_from_i64(value: i64, currency_idx in 0..11usize) {
        let currency = test_currencies()[currency_idx];
        let money = FinMoney::from_i64(value, currency);
        prop_assert_eq!(money.get_amount(), Decimal::from(value));
        prop_assert_eq!(money.get_currency(), currency);
    }

    // Feature: project-improvements, Property 4: from_f64 and TryFrom correctness
    // **Validates: Requirements 4.2, 4.3, 4.4**
    #[test]
    fn p4_from_f64_finite(
        // Use i32 mapped to f64 to avoid problematic f64 values
        raw in -1_000_000i32..1_000_000i32,
        currency_idx in 0..11usize,
    ) {
        let value = raw as f64 / 100.0;
        let currency = test_currencies()[currency_idx];
        let from_f64 = FinMoney::from_f64(value, currency);
        let try_from = FinMoney::try_from((value, currency));
        // Both should succeed for finite values
        prop_assert!(from_f64.is_ok(), "from_f64 failed for {}", value);
        prop_assert!(try_from.is_ok(), "try_from failed for {}", value);
        // Both should return the same amount
        prop_assert_eq!(
            from_f64.unwrap().get_amount(),
            try_from.unwrap().get_amount()
        );
    }

    #[test]
    fn p4_from_f64_nan_inf(currency_idx in 0..11usize) {
        let currency = test_currencies()[currency_idx];
        prop_assert!(FinMoney::from_f64(f64::NAN, currency).is_err());
        prop_assert!(FinMoney::from_f64(f64::INFINITY, currency).is_err());
        prop_assert!(FinMoney::from_f64(f64::NEG_INFINITY, currency).is_err());
        prop_assert!(FinMoney::try_from((f64::NAN, currency)).is_err());
        prop_assert!(FinMoney::try_from((f64::INFINITY, currency)).is_err());
        prop_assert!(FinMoney::try_from((f64::NEG_INFINITY, currency)).is_err());
    }

    // Feature: project-improvements, Property 5: Allocate — sum invariant
    // **Validates: Requirements 5.2**
    #[test]
    fn p5_allocate_sum_invariant(
        // Use a reasonable positive amount to avoid overflow in allocate
        amount_raw in 1i64..=1_000_000i64,
        currency_idx in 0..11usize,
        weights in prop::collection::vec(1i64..=1000i64, 1..=10),
    ) {
        let currency = test_currencies()[currency_idx];
        let precision = currency.get_precision() as u32;
        let amount = Decimal::new(amount_raw, precision.min(4));
        let money = FinMoney::new(amount, currency);
        let dec_weights: Vec<Decimal> = weights.iter().map(|&w| Decimal::from(w)).collect();

        let parts = money.allocate(&dec_weights).unwrap();
        let sum: Decimal = parts.iter().map(|p| p.get_amount()).sum();
        prop_assert_eq!(sum, amount, "Sum of parts {} != original amount {}", sum, amount);
    }

    // Feature: project-improvements, Property 6: Allocate — proportionality
    // **Validates: Requirements 5.1**
    #[test]
    fn p6_allocate_proportionality(
        amount_raw in 1000i64..=1_000_000i64,
        currency_idx in 0..11usize,
        weights in prop::collection::vec(1i64..=1000i64, 2..=5),
    ) {
        let currency = test_currencies()[currency_idx];
        let precision = currency.get_precision() as u32;
        // Use precision capped at 4 to keep amounts reasonable
        let effective_precision = precision.min(4);
        let amount = Decimal::new(amount_raw, effective_precision);
        let money = FinMoney::new(amount, currency);
        let dec_weights: Vec<Decimal> = weights.iter().map(|&w| Decimal::from(w)).collect();
        let total_weight: Decimal = dec_weights.iter().copied().sum();

        let parts = money.allocate(&dec_weights).unwrap();
        let total_amount: Decimal = parts.iter().map(|p| p.get_amount()).sum();
        let step = Decimal::new(1, precision);

        for (i, part) in parts.iter().enumerate() {
            let expected_ratio = dec_weights[i] / total_weight;
            let actual_ratio = if total_amount.is_zero() {
                Decimal::ZERO
            } else {
                part.get_amount() / total_amount
            };
            let diff = (actual_ratio - expected_ratio).abs();
            // The difference should be at most one step relative to total
            let tolerance = if total_amount.is_zero() {
                step
            } else {
                step / total_amount + Decimal::new(1, 10) // small epsilon for rounding
            };
            prop_assert!(
                diff <= tolerance,
                "Part {} ratio diff {} exceeds tolerance {} (actual={}, expected={})",
                i, diff, tolerance, actual_ratio, expected_ratio
            );
        }
    }

    // Feature: project-improvements, Property 9: Sum and try_sum consistency
    // **Validates: Requirements 9.1, 9.3**
    #[test]
    fn p9_sum_try_sum_consistency(
        amounts in prop::collection::vec(-1_000_000i64..=1_000_000i64, 1..=20),
        currency_idx in 0..11usize,
    ) {
        let currency = test_currencies()[currency_idx];
        let moneys: Vec<FinMoney> = amounts
            .iter()
            .map(|&v| FinMoney::new(Decimal::from(v), currency))
            .collect();

        // fold via plus_money
        let fold_result = moneys[1..].iter().try_fold(moneys[0], |acc, &item| acc.plus_money(item));

        // try_sum
        let try_sum_result = FinMoney::try_sum(moneys.clone().into_iter());

        // iter().sum()
        let sum_result: FinMoney = moneys.clone().into_iter().sum();

        match (fold_result, try_sum_result) {
            (Ok(fold_val), Ok(try_sum_val)) => {
                prop_assert_eq!(fold_val.get_amount(), try_sum_val.get_amount());
                prop_assert_eq!(fold_val.get_amount(), sum_result.get_amount());
            }
            (Err(_), Err(_)) => {
                // Both failed (overflow) — consistent
            }
            (Ok(f), Err(e)) => {
                prop_assert!(false, "fold succeeded ({}) but try_sum failed ({:?})", f, e);
            }
            (Err(e), Ok(t)) => {
                prop_assert!(false, "fold failed ({:?}) but try_sum succeeded ({})", e, t);
            }
        }
    }

    // Feature: project-improvements, Property 10: Error predicates
    // **Validates: Requirements 10.3**
    #[test]
    fn p10_error_predicates(error in arb_finmoney_error()) {
        let is_cm = error.is_currency_mismatch();
        let is_dz = error.is_division_by_zero();
        let is_ov = error.is_overflow();

        match &error {
            FinMoneyError::CurrencyMismatch { .. } => {
                prop_assert!(is_cm);
                prop_assert!(!is_dz);
                prop_assert!(!is_ov);
            }
            FinMoneyError::DivisionByZero => {
                prop_assert!(!is_cm);
                prop_assert!(is_dz);
                prop_assert!(!is_ov);
            }
            FinMoneyError::ArithmeticOverflow => {
                prop_assert!(!is_cm);
                prop_assert!(!is_dz);
                prop_assert!(is_ov);
            }
            // For all other variants, all three predicates should be false
            _ => {
                prop_assert!(!is_cm, "is_currency_mismatch should be false for {:?}", error);
                prop_assert!(!is_dz, "is_division_by_zero should be false for {:?}", error);
                prop_assert!(!is_ov, "is_overflow should be false for {:?}", error);
            }
        }
    }

    // Feature: project-improvements, Property 11: convert_to correctness and precision
    // **Validates: Requirements 11.1, 11.3**
    #[test]
    fn p11_convert_to(
        amount_raw in -1_000_000i64..=1_000_000i64,
        source_idx in 0..11usize,
        target_idx in 0..11usize,
        rate_raw in 1i64..=100_000i64,
        strategy in arb_rounding_strategy(),
    ) {
        let source_currency = test_currencies()[source_idx];
        let target_currency = test_currencies()[target_idx];
        let money = FinMoney::new(Decimal::from(amount_raw), source_currency);
        let rate = Decimal::from(rate_raw) / Decimal::from(10_000);

        let result = money.convert_to(target_currency, rate, strategy);
        prop_assert!(result.is_ok(), "convert_to failed: {:?}", result);

        let converted = result.unwrap();
        prop_assert_eq!(
            converted.get_currency(),
            target_currency,
            "Currency mismatch: expected {:?}, got {:?}",
            target_currency,
            converted.get_currency()
        );

        // Check decimal places don't exceed target precision
        let target_precision = target_currency.get_precision() as u32;
        let amount_str = converted.get_amount().to_string();
        if let Some((_int, frac)) = amount_str.split_once('.') {
            // Trim trailing zeros for comparison
            let trimmed = frac.trim_end_matches('0');
            prop_assert!(
                trimmed.len() as u32 <= target_precision,
                "Decimal places {} exceed target precision {} for amount {}",
                trimmed.len(),
                target_precision,
                converted.get_amount()
            );
        }
    }

    // Feature: project-improvements, Property 12: exchange_rate_to round-trip
    // **Validates: Requirements 11.4**
    #[test]
    fn p12_exchange_rate_to(
        a_raw in 1i64..=10_000i64,
        // Generate b as a multiple of a to ensure exact division
        multiplier in 1i64..=1_000i64,
    ) {
        let b_raw = a_raw * multiplier;
        let a = FinMoney::new(Decimal::from(a_raw), test_currencies()[0]);
        let b = FinMoney::new(Decimal::from(b_raw), test_currencies()[1]);

        let rate = a.exchange_rate_to(b).unwrap();
        let product = a.get_amount() * rate;
        prop_assert_eq!(
            product,
            b.get_amount(),
            "a.amount * rate ({} * {}) = {} != b.amount {}",
            a.get_amount(),
            rate,
            product,
            b.get_amount()
        );
    }

    // Feature: project-improvements, Property 13: format_with_separator contains currency code and separators
    // **Validates: Requirements 12.1**
    #[test]
    fn p13_format_with_separator(
        // Generate amounts >= 1000 to ensure thousands separator is present
        amount_raw in 1_000i64..=999_999_999i64,
        currency_idx in 0..11usize,
    ) {
        let currency = test_currencies()[currency_idx];
        let money = FinMoney::new(Decimal::from(amount_raw), currency);
        let formatted = money.format_with_separator(',', '.');

        let code = currency.get_code();
        // Must end with " CODE"
        prop_assert!(
            formatted.ends_with(&format!(" {}", code)),
            "Formatted '{}' must end with ' {}'", formatted, code
        );
        // Must contain thousands separator for amounts >= 1000
        prop_assert!(
            formatted.contains(','),
            "Formatted '{}' must contain thousands separator ',' for amount {}",
            formatted, amount_raw
        );
    }

    // Feature: project-improvements, Property 14: format_padded pads with zeros
    // **Validates: Requirements 12.3**
    #[test]
    fn p14_format_padded(
        amount_raw in -1_000_000i64..=1_000_000i64,
        currency_idx in 0..11usize,
        dp in 0u32..=10u32,
    ) {
        let currency = test_currencies()[currency_idx];
        let money = FinMoney::new(Decimal::from(amount_raw), currency);
        let formatted = money.format_padded(dp);

        let code = currency.get_code();
        // Must end with " CODE"
        prop_assert!(
            formatted.ends_with(&format!(" {}", code)),
            "Formatted '{}' must end with ' {}'", formatted, code
        );

        // Extract the numeric part (before the last space + code)
        let numeric_part = formatted.strip_suffix(&format!(" {}", code)).unwrap();

        if dp == 0 {
            // Should not contain a decimal point
            prop_assert!(
                !numeric_part.contains('.'),
                "format_padded(0) should not contain decimal point, got '{}'", formatted
            );
        } else {
            // Should contain exactly dp digits after the decimal point
            let parts: Vec<&str> = numeric_part.split('.').collect();
            prop_assert_eq!(
                parts.len(), 2,
                "Expected exactly one decimal point in '{}', got {} parts", numeric_part, parts.len()
            );
            let frac = parts[1];
            prop_assert_eq!(
                frac.len() as u32, dp,
                "Expected {} decimal places in '{}', got {}", dp, formatted, frac.len()
            );
        }
    }
}
