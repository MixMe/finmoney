# finmoney

[![Crates.io](https://img.shields.io/crates/v/finmoney.svg)](https://crates.io/crates/finmoney)
[![Documentation](https://docs.rs/finmoney/badge.svg)](https://docs.rs/finmoney)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A precise, panic-free FinMoney library for Rust. finmoney provides safe monetary arithmetic, currency-aware values, configurable rounding strategies, and exchange-grade tick handling. Designed for trading systems, bots, and financial apps where correctness and determinism matter.

## Features

- **Precise arithmetic**: Built on `rust_decimal` for exact decimal calculations
- **Currency safety**: Prevents mixing different currencies in operations
- **Configurable rounding**: Multiple rounding strategies for different use cases
- **Tick handling**: Exchange-grade price/quantity rounding to valid tick sizes
- **Zero panics**: All operations return `Result` types for error handling
- **Serde support**: Optional serialization/deserialization (feature-gated)
- **Modern Rust**: Uses Rust 2024 edition for the latest language features

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
finmoney = "1.0.2"

# For serialization support
finmoney = { version = "1.0.2", features = ["serde"] }
```

## Basic Usage

```rust
use finmoney::{FinMoney, FinmoneyCurrency, MoneyRoundingStrategy};
use rust_decimal_macros::dec;

// Create currencies
let usd = FinmoneyCurrency::new(1, "USD".to_string(), Some("US Dollar".to_string()), 2)?;
let btc = FinmoneyCurrency::new(2, "BTC".to_string(), Some("Bitcoin".to_string()), 8)?;

// Or use predefined currencies
let usd = FinmoneyCurrency::USD;
let eur = FinmoneyCurrency::EUR;

// Create FinMoney values
let price = FinMoney::new(dec!(10.50), usd);
let tax = FinMoney::new(dec!(1.05), usd);

// Perform arithmetic (returns Result for safety)
let total = (price + tax)?;
println!("{}", total); // 11.55 USD

// Multiply by decimal
let doubled = price * dec!(2);
println!("{}", doubled); // 21.00 USD

// Division with rounding
let divided = price.divided_by_decimal(dec!(3), MoneyRoundingStrategy::MidpointNearestEven)?;
println!("{}", divided); // 3.50 USD
```

## Currency Safety

finmoney prevents mixing different currencies:

```rust
let usd_amount = FinMoney::new(dec!(100), FinmoneyCurrency::USD);
let eur_amount = FinMoney::new(dec!(85), FinmoneyCurrency::EUR);

// This will return an error
match usd_amount + eur_amount {
    Ok(result) => println!("Sum: {}", result),
    Err(e) => println!("Error: {}", e), // Currency mismatch: expected USD, got EUR
}
```

## Tick Handling for Trading

Perfect for exchange trading where prices must conform to specific tick sizes:

```rust
let price = FinMoney::new(dec!(10.567), finmoneyCurrency::USD);

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
let amount = FinMoney::new(dec!(10.555), finmoneyCurrency::USD);

// Banker's rounding (default)
let rounded1 = amount.round_dp_with_strategy(2, MoneyRoundingStrategy::MidpointNearestEven);

// Always round away from zero
let rounded2 = amount.round_dp_with_strategy(2, MoneyRoundingStrategy::MidpointAwayFromZero);

// Always round toward zero
let rounded3 = amount.round_dp_with_strategy(2, MoneyRoundingStrategy::MidpointTowardZero);
```

## Percentage Calculations

```rust
let initial = FinMoney::new(dec!(100), finmoneyCurrency::USD);
let current = FinMoney::new(dec!(110), finmoneyCurrency::USD);

// Calculate percentage change
let change = current.percent_change_from(initial)?;
println!("Change: {}%", change); // Change: 10%

// Or use static method
let change = FinMoney::percent_change(initial, current)?;
```

## Comparison Operations

```rust
let price1 = FinMoney::new(dec!(10.50), finmoneyCurrency::USD);
let price2 = FinMoney::new(dec!(9.75), finmoneyCurrency::USD);

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
let FinMoney = FinMoney::new(dec!(-15.75), finmoneyCurrency::USD);

println!("Is zero: {}", FinMoney.is_zero());
println!("Is positive: {}", FinMoney.is_positive());
println!("Is negative: {}", FinMoney.is_negative());
println!("Has fraction: {}", FinMoney.has_fraction());
println!("Is integer: {}", FinMoney.is_integer());

// Mathematical operations
let abs_FinMoney = FinMoney.abs();        // 15.75 USD
let neg_FinMoney = FinMoney.negated();    // 15.75 USD
let floor_FinMoney = FinMoney.floor();    // -16.00 USD
let ceil_FinMoney = FinMoney.ceil();      // -15.00 USD
```

## Error Handling

All operations that can fail return `Result<T, MoneyError>`:

```rust
use finmoney::MoneyError;

let result = FinMoney1.divided_by_decimal(dec!(0), MoneyRoundingStrategy::default());
match result {
    Ok(value) => println!("Result: {}", value),
    Err(MoneyError::DivisionByZero) => println!("Cannot divide by zero"),
    Err(MoneyError::CurrencyMismatch { expected, actual }) => {
        println!("Currency mismatch: expected {}, got {}", expected, actual);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Predefined Currencies

Common currencies are available as constants:

```rust
let usd_FinMoney = FinMoney::new(dec!(100), finmoneyCurrency::USD);  // 2 decimal places
let eur_FinMoney = FinMoney::new(dec!(85), finmoneyCurrency::EUR);   // 2 decimal places
let btc_FinMoney = FinMoney::new(dec!(0.001), finmoneyCurrency::BTC); // 8 decimal places
let eth_FinMoney = FinMoney::new(dec!(0.1), finmoneyCurrency::ETH);   // 18 decimal places
```

## Serde Support

Enable the `serde` feature for serialization support:

```toml
[dependencies]
finmoney = { version = "0.1", features = ["serde"] }
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Order {
    price: FinMoney,
    quantity: FinMoney,
}

let order = Order {
    price: FinMoney::new(dec!(10.50), finmoneyCurrency::USD),
    quantity: FinMoney::new(dec!(5), finmoneyCurrency::USD),
};

let json = serde_json::to_string(&order)?;
let deserialized: Order = serde_json::from_str(&json)?;
```

## Performance

finmoney is built on `rust_decimal` which provides excellent performance for financial calculations. All operations are designed to be allocation-free where possible.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
