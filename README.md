# finmoney

[![Crates.io](https://img.shields.io/crates/v/finmoney.svg)](https://crates.io/crates/finmoney)
[![Documentation](https://docs.rs/finmoney/badge.svg)](https://docs.rs/finmoney)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A precise, panic-free FinMoney library for Rust. finmoney provides safe monetary arithmetic, currency-aware values, configurable rounding strategies, and exchange-grade tick handling. Designed for trading systems, bots, and financial apps where correctness and determinism matter.

## Features

- **Precise arithmetic**: Built on `rust_decimal` for exact decimal calculations with checked overflow detection
- **Currency safety**: Prevents mixing different currencies in operations
- **Configurable rounding**: Multiple rounding strategies for different use cases
- **Tick handling**: Exchange-grade price/quantity rounding to valid tick sizes
- **Zero panics**: All operations return `Result` types for error handling
- **11 predefined currencies**: USD, EUR, BTC, ETH, GBP, JPY, CHF, CNY, RUB, USDT, SOL
- **Money allocation**: Split amounts by weights with zero-loss remainder distribution
- **Currency conversion**: `convert_to` and `exchange_rate_to` for multi-currency workflows
- **Iterator support**: `Sum` trait and `try_sum` for idiomatic collection operations
- **Flexible formatting**: `format_with_separator` and `format_padded` for display
- **Convenient constructors**: `from_i64`, `from_f64`, `TryFrom<(f64, FinMoneyCurrency)>`
- **Error predicates**: `is_currency_mismatch()`, `is_division_by_zero()`, `is_overflow()`
- **Serde support**: Optional serialization/deserialization (feature-gated)
- **Modern Rust**: Uses Rust 2024 edition, no unsafe code

## Requirements

- Rust 1.90 or later (Rust 2024 edition)

## Rust 2024 Edition Benefits

This library uses the Rust 2024 edition, which provides:
- Improved error messages and diagnostics
- Better async/await ergonomics
- Enhanced pattern matching capabilities
- More consistent and intuitive syntax
- Latest language features and optimizations

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
finmoney = "3.0.0"

# For serialization support
finmoney = { version = "3.0.0", features = ["serde"] }
```

## Basic Usage

```rust
use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyRoundingStrategy};
use rust_decimal_macros::dec;

// Create currencies
let usd = FinMoneyCurrency::new(1, "USD", Some("US Dollar"), 2)?;
let btc = FinMoneyCurrency::new(2, "BTC", Some("Bitcoin"), 8)?;

// Or use predefined currencies
let usd = FinMoneyCurrency::USD;
let eur = FinMoneyCurrency::EUR;

// Create FinMoney values
let price = FinMoney::new(dec!(10.50), usd);
let tax = FinMoney::new(dec!(1.05), usd);

// Perform arithmetic (returns Result for safety)
let total = (price + tax)?;
println!("{}", total); // 11.55 USD

// Multiply by decimal (returns Result, both directions work)
let doubled = (price * dec!(2))?;
let also_doubled = (dec!(2) * price)?;
println!("{}", doubled); // 21.00 USD

// Division with rounding
let divided = price.divided_by_decimal(dec!(3), FinMoneyRoundingStrategy::MidpointNearestEven)?;
println!("{}", divided); // 3.50 USD
```

## Performance-Optimized Currency Creation

For performance-critical applications, use `new_from_tiny` with pre-calculated `TinyAsciiStr` values:

```rust
use finmoney::FinMoneyCurrency;
use tinystr::TinyAsciiStr;

// Pre-calculate TinyAsciiStr values (more efficient)
let code: TinyAsciiStr<16> = "USD".parse().unwrap();
let name: TinyAsciiStr<52> = "US Dollar".parse().unwrap();

// Create currency without string parsing overhead
let usd = FinMoneyCurrency::new_from_tiny(1, code, Some(name), 2)?;
```

## Currency Safety

finmoney prevents mixing different currencies:

```rust
let usd_amount = FinMoney::new(dec!(100), FinMoneyCurrency::USD);
let eur_amount = FinMoney::new(dec!(85), FinMoneyCurrency::EUR);

// This will return an error
match usd_amount + eur_amount {
    Ok(result) => println!("Sum: {}", result),
    Err(e) => println!("Error: {}", e), // Currency mismatch: expected USD, got EUR
}
```

## Tick Handling for Trading

Perfect for exchange trading where prices must conform to specific tick sizes:

```rust
let price = FinMoney::new(dec!(10.567), FinMoneyCurrency::USD);

// Round to nearest 0.25 tick
let rounded = price.to_tick_nearest(dec!(0.25))?;
println!("{}", rounded); // 10.50 USD

// Round down (floor)
let floor_price = price.to_tick_down(dec!(0.25))?;
println!("{}", floor_price); // 10.50 USD

// Round up (ceiling)
let ceil_price = price.to_tick_up(dec!(0.25))?;
println!("{}", ceil_price); // 10.75 USD

// Check if price is valid for tick size
if price.is_multiple_of_tick(dec!(0.01)) {
    println!("Price is valid for 0.01 tick size");
}
```

## Rounding Strategies

Multiple rounding strategies are available:

```rust
let amount = FinMoney::new(dec!(10.555), FinMoneyCurrency::USD);

// Banker's rounding (default)
let rounded1 = amount.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointNearestEven);

// Always round away from zero
let rounded2 = amount.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointAwayFromZero);

// Always round toward zero
let rounded3 = amount.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointTowardZero);
```

## Percentage Calculations

```rust
let initial = FinMoney::new(dec!(100), FinMoneyCurrency::USD);
let current = FinMoney::new(dec!(110), FinMoneyCurrency::USD);

// Calculate percentage change
let change = current.percent_change_from(initial)?;
println!("Change: {}%", change); // Change: 10%

// Or use static method
let change = FinMoney::percent_change(initial, current)?;
```

## Comparison Operations

```rust
let price1 = FinMoney::new(dec!(10.50), FinMoneyCurrency::USD);
let price2 = FinMoney::new(dec!(9.75), FinMoneyCurrency::USD);

// Safe comparisons (returns Result)
if price1.is_greater_than(price2)? {
    println!("Price1 is higher");
}

// Min/max operations
let lower = price1.min(price2)?;
let higher = price1.max(price2)?;

// Direct decimal comparisons (no Result needed)
if price1.is_greater_than_decimal(dec!(10)) {
    println!("Price is above 10");
}
```

## Properties and Utilities

```rust
let money = FinMoney::new(dec!(-15.75), FinMoneyCurrency::USD);

println!("Is zero: {}", money.is_zero());
println!("Is positive: {}", money.is_positive());
println!("Is negative: {}", money.is_negative());
println!("Has fraction: {}", money.has_fraction());
println!("Is integer: {}", money.is_integer());

// Mathematical operations
let abs_money = money.abs();        // 15.75 USD
let neg_money = -money;             // 15.75 USD
let floor_money = money.floor();    // -16.00 USD
let ceil_money = money.ceil();      // -15.00 USD
```

## Error Handling

All operations that can fail return `Result<T, FinMoneyError>`:

```rust
use finmoney::FinMoneyError;

let result = money1.divided_by_decimal(dec!(0), FinMoneyRoundingStrategy::default());
match result {
    Ok(value) => println!("Result: {}", value),
    Err(FinMoneyError::DivisionByZero) => println!("Cannot divide by zero"),
    Err(FinMoneyError::CurrencyMismatch { expected, actual }) => {
        println!("Currency mismatch: expected {}, got {}", expected, actual);
    }
    Err(FinMoneyError::ArithmeticOverflow) => println!("Overflow detected"),
    Err(e) => println!("Other error: {}", e),
}

// Convenient error predicates (no pattern matching needed)
if let Err(e) = result {
    if e.is_currency_mismatch() { /* ... */ }
    if e.is_division_by_zero() { /* ... */ }
    if e.is_overflow() { /* ... */ }
}
```

## Predefined Currencies

11 common currencies are available as constants:

```rust
let usd = FinMoney::new(dec!(100), FinMoneyCurrency::USD);    // precision 2
let eur = FinMoney::new(dec!(85), FinMoneyCurrency::EUR);     // precision 2
let btc = FinMoney::new(dec!(0.001), FinMoneyCurrency::BTC);  // precision 8
let eth = FinMoney::new(dec!(0.1), FinMoneyCurrency::ETH);    // precision 18
let gbp = FinMoney::new(dec!(80), FinMoneyCurrency::GBP);     // precision 2
let jpy = FinMoney::new(dec!(15000), FinMoneyCurrency::JPY);  // precision 0
let chf = FinMoney::new(dec!(90), FinMoneyCurrency::CHF);     // precision 2
let cny = FinMoney::new(dec!(720), FinMoneyCurrency::CNY);    // precision 2
let rub = FinMoney::new(dec!(9500), FinMoneyCurrency::RUB);   // precision 2
let usdt = FinMoney::new(dec!(100), FinMoneyCurrency::USDT);  // precision 6
let sol = FinMoney::new(dec!(0.5), FinMoneyCurrency::SOL);    // precision 9

// Get all predefined currencies
let all = FinMoneyCurrency::all_predefined(); // &[FinMoneyCurrency] with 11 entries

// Display trait
println!("{}", FinMoneyCurrency::USD); // "USD (US Dollar)"
println!("{}", FinMoneyCurrency::BTC); // "BTC"
```

## Convenient Constructors

```rust
// From i64
let money = FinMoney::from_i64(100, FinMoneyCurrency::USD);

// From f64 (returns Result — NaN/Infinity produce errors)
let money = FinMoney::from_f64(99.95, FinMoneyCurrency::USD)?;

// Via TryFrom
let money = FinMoney::try_from((99.95, FinMoneyCurrency::USD))?;
```

## Money Allocation

Split a monetary amount by weights with zero-loss remainder distribution:

```rust
let total = FinMoney::new(dec!(100), FinMoneyCurrency::USD);
let parts = total.allocate(&[dec!(1), dec!(1), dec!(1)])?;
// [33.34 USD, 33.33 USD, 33.33 USD] — remainder goes to first parts
assert_eq!(parts.iter().map(|p| p.get_amount()).sum::<Decimal>(), dec!(100));
```

## Iterator Support

```rust
let amounts = vec![
    FinMoney::new(dec!(10), FinMoneyCurrency::USD),
    FinMoney::new(dec!(20), FinMoneyCurrency::USD),
    FinMoney::new(dec!(30), FinMoneyCurrency::USD),
];

// Sum trait (panics on currency mismatch or empty iterator)
let total: FinMoney = amounts.clone().into_iter().sum();

// Safe alternative
let total = FinMoney::try_sum(amounts.into_iter())?;
```

## Currency Conversion

```rust
let usd = FinMoney::new(dec!(100), FinMoneyCurrency::USD);

// Convert at a given rate
let eur = usd.convert_to(
    FinMoneyCurrency::EUR,
    dec!(0.92),
    FinMoneyRoundingStrategy::MidpointNearestEven,
)?;
println!("{}", eur); // 92.00 EUR

// Calculate exchange rate between two amounts
let rate = usd.exchange_rate_to(eur)?;
println!("Rate: {}", rate); // 0.92
```

## Formatting

```rust
let money = FinMoney::new(dec!(1234567.89), FinMoneyCurrency::USD);

// Custom separators
println!("{}", money.format_with_separator(',', '.')); // "1,234,567.89 USD"
println!("{}", money.format_with_separator('.', ',')); // "1.234.567,89 USD"

// Pad to fixed decimal places
let m = FinMoney::new(dec!(10.5), FinMoneyCurrency::USD);
println!("{}", m.format_padded(4)); // "10.5000 USD"
```

## Serde Support

Enable the `serde` feature for serialization support:

```toml
[dependencies]
finmoney = { version = "3.0.0", features = ["serde"] }
```
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Order {
    price: FinMoney,
    quantity: FinMoney,
}

let order = Order {
    price: FinMoney::new(dec!(10.50), FinMoneyCurrency::USD),
    quantity: FinMoney::new(dec!(5), FinMoneyCurrency::USD),
};

let json = serde_json::to_string(&order)?;
let deserialized: Order = serde_json::from_str(&json)?;
```

## Migration Guide

This section covers breaking changes introduced in v2.x. Update your code accordingly.

### 1. Checked arithmetic: `plus_decimal` / `minus_decimal`

These methods now return `Result<FinMoney, FinMoneyError>` instead of `FinMoney` to detect overflow.

Before:
```rust
let result = money.plus_decimal(dec!(5));    // FinMoney
let result = money.minus_decimal(dec!(3));   // FinMoney
```

After:
```rust
let result = money.plus_decimal(dec!(5))?;   // Result<FinMoney, FinMoneyError>
let result = money.minus_decimal(dec!(3))?;  // Result<FinMoney, FinMoneyError>
```

### 2. `Mul<Decimal> for FinMoney`

The `*` operator with `Decimal` now returns `Result<FinMoney, FinMoneyError>` instead of `FinMoney`.

Before:
```rust
let doubled = money * dec!(2);               // FinMoney
```

After:
```rust
let doubled = (money * dec!(2))?;            // Result<FinMoney, FinMoneyError>
```

### 3. `Mul<FinMoney> for Decimal`

Same change — multiplying `Decimal * FinMoney` now returns `Result`.

Before:
```rust
let doubled = dec!(2) * money;               // FinMoney
```

After:
```rust
let doubled = (dec!(2) * money)?;            // Result<FinMoney, FinMoneyError>
```

### 4. `multiplied_by_decimal`

Before:
```rust
let result = money.multiplied_by_decimal(dec!(1.5));  // FinMoney
```

After:
```rust
let result = money.multiplied_by_decimal(dec!(1.5))?; // Result<FinMoney, FinMoneyError>
```

All arithmetic operations now consistently return `Result`, making overflow detection explicit. The `ArithmeticOverflow` error variant is returned when the result exceeds the `Decimal` range.

## Performance

finmoney is built on `rust_decimal` which provides excellent performance for financial calculations. All operations are designed to be allocation-free where possible.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
