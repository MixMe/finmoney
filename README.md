# finmoney

[![Crates.io](https://img.shields.io/crates/v/finmoney.svg)](https://crates.io/crates/finmoney)
[![Documentation](https://docs.rs/finmoney/badge.svg)](https://docs.rs/finmoney)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A precise money library for Rust built for trading systems. Provides currency-aware arithmetic,
exchange-grade tick handling, configurable rounding, and fair allocation. Built on `rust_decimal`
for exact decimal calculations.

## Design Principles

- **Ergonomic by default.** Comparisons (`<`, `>`, `<=`, `>=`), scalar arithmetic (`* Decimal`),
  and in-place operations (`+=`, `-=`) return direct values — no `.unwrap()` ceremony on every line.
- **Result where it matters.** Only operations with real runtime failure modes (currency mismatch
  between two `FinMoney` values, division by zero, NaN/Infinity) return `Result`.
- **Checked variants available.** Every panicking method has a `Result`-returning counterpart
  (`unchecked_plus` ↔ `plus_money`, `* Decimal` ↔ `multiplied_by_decimal`).
- **Precise.** `rust_decimal` 128-bit decimals — no floating-point surprises.

## Requirements

- Rust 1.90+ (edition 2024)

## Quick Start

```toml
[dependencies]
finmoney = "4.0"

# With serde support
finmoney = { version = "4.0", features = ["serde"] }
```

```rust
use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyRoundingStrategy};
use rust_decimal_macros::dec;

let usd = FinMoneyCurrency::new(1, "USD", None::<&str>, 2).unwrap();

// Create money
let price = FinMoney::new(dec!(10.50), usd);
let tax = FinMoney::new(dec!(1.05), usd);

// Arithmetic: FinMoney + FinMoney returns Result (currency mismatch possible)
let total = (price + tax)?;                    // 11.55 USD

// Scalar multiply returns FinMoney directly (no currency check needed)
let doubled = price * dec!(2);                 // 21.00 USD

// Comparisons work with standard operators
if price > tax {
    println!("{} is more than {}", price, tax);
}
```

## Constructors

```rust
use finmoney::{FinMoney, FinMoneyCurrency};
use rust_decimal_macros::dec;

let usd = FinMoneyCurrency::new(1, "USD", None::<&str>, 2).unwrap();
let btc = FinMoneyCurrency::new(2, "BTC", None::<&str>, 8).unwrap();

// From Decimal
let m = FinMoney::new(dec!(10.50), usd);

// From integer
let m = FinMoney::from_i64(100, usd);

// From minor units (cents, satoshi)
let m = FinMoney::from_minor(1050, usd);       // 10.50 USD
let m = FinMoney::from_minor(100_000_000, btc); // 1.00000000 BTC

// From string
let m = FinMoney::from_str("10.50", usd)?;

// From f64 (returns Result — NaN/Infinity produce errors)
let m = FinMoney::from_f64(10.5, usd)?;

// Zero
let m = FinMoney::zero(usd);
```

## Arithmetic

```rust
let a = FinMoney::new(dec!(100), usd);
let b = FinMoney::new(dec!(30), usd);

// FinMoney ↔ FinMoney: returns Result (currency mismatch possible)
let sum  = (a + b)?;                           // 130 USD
let diff = (a - b)?;                           // 70 USD

// FinMoney ↔ Decimal: returns FinMoney directly (no currency check needed)
let added   = a + dec!(5);                     // 105 USD
let subbed  = a - dec!(5);                     // 95 USD
let tripled = a * dec!(3);                     // 300 USD
let also    = dec!(3) * a;                     // 300 USD

// In-place operators (all direct, panic only on currency mismatch for FinMoney)
let mut total = FinMoney::zero(usd);
total += a;                                    // FinMoney += FinMoney
total -= b;                                    // FinMoney -= FinMoney
total += dec!(10);                             // FinMoney += Decimal
total -= dec!(5);                              // FinMoney -= Decimal
total *= dec!(2);                              // FinMoney *= Decimal

// Division (returns Result — division by zero possible)
let half = a.divided_by_decimal(dec!(2), FinMoneyRoundingStrategy::MidpointNearestEven)?;

// Named methods (same as operators, available for readability)
let adjusted = a.plus_decimal(dec!(5));        // 105 USD
let reduced  = a.minus_decimal(dec!(5));       // 95 USD

// Unchecked variants for hot paths (panic on mismatch)
let fast_sum = a.unchecked_plus(b);            // 130 USD
let fast_diff = a.unchecked_minus(b);          // 70 USD
```

## Comparisons

`FinMoney` implements `PartialOrd` and `Ord`, so standard Rust operators work directly:

```rust
let price = FinMoney::new(dec!(10.50), usd);
let limit = FinMoney::new(dec!(9.75), usd);

// Standard operators (panic on currency mismatch)
if price > limit { /* ... */ }
if price <= limit { /* ... */ }

// Min/max
let lower = price.min(limit);                  // 9.75 USD
let higher = price.max(limit);                 // 10.50 USD

// Sorting works out of the box
let mut prices = vec![price, limit];
prices.sort();

// std::cmp::min/max
let m = std::cmp::min(price, limit);

// Named methods (also direct, panic on mismatch)
price.is_greater_than(limit);                  // true
price.is_less_than_or_equal(limit);            // false

// Checked variant (returns Result)
price.try_compare(limit)?;                     // Ordering::Greater

// Decimal comparisons (no currency check)
price.is_greater_than_decimal(dec!(10));        // true
```

## Tick Handling

Exchange-grade price/quantity rounding to valid tick sizes:

```rust
let price = FinMoney::new(dec!(10.567), usd);

// Round to tick
let nearest = price.to_tick_nearest(dec!(0.25))?;  // 10.50 USD
let floor   = price.to_tick_down(dec!(0.25))?;     // 10.50 USD
let ceil    = price.to_tick_up(dec!(0.25))?;       // 10.75 USD

// Validate tick alignment
price.is_multiple_of_tick(dec!(0.01));              // true

// Handles trailing zeros from exchange data (e.g., 0.000100000 → 0.0001)
let tick = rust_decimal::Decimal::new(100000, 9);   // 0.000100000
price.to_tick_nearest(tick)?;                       // uses fast path correctly
```

## Allocation and Splitting

```rust
let total = FinMoney::new(dec!(100.00), usd);

// Equal split with fair remainder distribution
let parts = total.split(3)?;
// [33.34 USD, 33.33 USD, 33.33 USD] — sum is always exact

// Weighted allocation
let parts = total.allocate(&[dec!(70), dec!(20), dec!(10)])?;
// [70.00 USD, 20.00 USD, 10.00 USD]
```

## Conversion

```rust
let money = FinMoney::new(dec!(123.45), usd);

// To minor units (cents, satoshi)
let cents = money.to_minor_units();                 // 12345

// To f64 (explicitly lossy — for UI/metrics only)
let f = money.to_f64_lossy();                       // 123.45

// Currency conversion at a rate
let eur_currency = FinMoneyCurrency::new(2, "EUR", None::<&str>, 2).unwrap();
let eur = money.convert_to(
    eur_currency,
    dec!(0.92),
    FinMoneyRoundingStrategy::MidpointNearestEven,
)?;                                                 // 113.57 EUR

// Calculate exchange rate
let rate = money.exchange_rate_to(eur)?;            // 0.92...
```

## Formatting

```rust
let m = FinMoney::new(dec!(1234567.89), usd);

println!("{}", m);                                  // 1234567.89 USD
println!("{}", m.format_with_separator(',', '.'));   // 1,234,567.89 USD
println!("{}", m.format_padded(4));                  // 1234567.8900 USD
```

## Rounding Strategies

```rust
let amount = FinMoney::new(dec!(10.555), usd);

amount.rounded(FinMoneyRoundingStrategy::MidpointNearestEven);   // 10.56 (banker's)
amount.rounded(FinMoneyRoundingStrategy::MidpointAwayFromZero);  // 10.56
amount.rounded(FinMoneyRoundingStrategy::ToZero);                // 10.55
amount.rounded(FinMoneyRoundingStrategy::ToNegativeInfinity);    // 10.55 (floor)
amount.rounded(FinMoneyRoundingStrategy::ToPositiveInfinity);    // 10.56 (ceil)
```

## Currencies

No predefined currencies — create any currency for any domain:

```rust
// Standard constructor (validates inputs, returns Result)
let usd = FinMoneyCurrency::new(1, "USD", Some("US Dollar"), 2)?;
let btc = FinMoneyCurrency::new(2, "BTC", Some("Bitcoin"), 8)?;
let gold = FinMoneyCurrency::new(100, "GOLD", Some("Gold Token"), 4)?;

// Lenient constructor (sanitizes invalid inputs, never fails)
let safe = FinMoneyCurrency::new_sanitized(1, "USD".into(), None, 2);

// High-performance constructor (pre-parsed TinyAsciiStr, skips parsing)
let code: tinystr::TinyAsciiStr<16> = "USD".parse().unwrap();
let fast = FinMoneyCurrency::new_from_tiny(1, code, None, 2)?;
```

## Error Handling

```rust
use finmoney::FinMoneyError;

match usd_money + eur_money {
    Ok(sum) => println!("{}", sum),
    Err(FinMoneyError::CurrencyMismatch { expected, actual }) => {
        println!("Cannot mix {} and {}", expected, actual);
    }
    Err(e) => println!("Error: {}", e),
}

// Predicates for matching
if let Err(e) = result {
    e.is_currency_mismatch();
    e.is_division_by_zero();
    e.is_overflow();
}
```

## API Design: When Result vs Direct

| Returns `Result` | Returns direct value |
|-------------------|---------------------|
| `FinMoney + FinMoney` (currency mismatch) | `FinMoney + Decimal`, `FinMoney - Decimal` |
| `FinMoney - FinMoney` (currency mismatch) | `FinMoney * Decimal`, `Decimal * FinMoney` |
| `divided_by_decimal` (division by zero) | `+=` / `-=` (FinMoney and Decimal) |
| `divided_by_money` (both) | `*=` Decimal, `-` (Neg) |
| `convert_to` (invalid rate) | `<` / `>` / `<=` / `>=` (PartialOrd) |
| `from_f64` / `from_str` (invalid input) | `min` / `max` / `compare` |
| `allocate` / `split` (zero weights/parts) | `abs` / `floor` / `ceil` / `trunc` |
| `sqrt` (negative amount) | `rescale` |
| `to_tick` (invalid tick ≤ 0) | `unchecked_plus` / `unchecked_minus` / `unchecked_mul` |

Every panicking method has a checked counterpart: `plus_money()` for `unchecked_plus()`,
`multiplied_by_decimal()` for `*`, `try_compare()` for `compare()`.

`FinMoney` implements: `Debug Clone Copy PartialEq Eq Hash PartialOrd Ord Display Neg Add Sub Mul AddAssign SubAssign MulAssign Sum`.
Does NOT implement `Default` — use `FinMoney::zero(currency)` for explicit construction.

## Migration from v3 to v4

**Comparisons return direct values (not Result):**
```rust
// v3:
if price.is_greater_than(other)? { ... }
let min = a.min(b)?;

// v4:
if price > other { ... }              // PartialOrd
if price.is_greater_than(other) { ... } // also works, returns bool
let min = a.min(b);                   // returns FinMoney
```

**Scalar arithmetic returns direct values (and has operators):**
```rust
// v3:
let result = money.plus_decimal(dec!(5))?;
let doubled = (money * dec!(2))?;
let rescaled = money.rescale(4)?;

// v4:
let result = money + dec!(5);               // Add<Decimal> operator
let doubled = money * dec!(2);              // Mul<Decimal> operator
let rescaled = money.rescale(4);            // direct FinMoney
money += dec!(5);                           // AddAssign<Decimal>
money -= dec!(3);                           // SubAssign<Decimal>
money *= dec!(2);                           // MulAssign<Decimal>
```

**New traits:**
- `Hash` — normalized hashing (10.50 and 10.5 hash equally), enables `HashSet<FinMoney>`
- `PartialOrd` / `Ord` — standard `<`, `>`, `<=`, `>=` operators and `sort()`
- `Add<Decimal>` / `Sub<Decimal>` — `money + dec!(5)` works directly
- `AddAssign` / `SubAssign` / `MulAssign` for both `FinMoney` and `Decimal`
- Removed `Default` — prevents silent creation of UNDEFINED currency values

**New methods:**
- `from_minor()` / `to_minor_units()` — minor unit conversion (cents, satoshi)
- `from_str()` — parse from decimal string
- `to_f64_lossy()` — explicit lossy f64 conversion for UI/metrics
- `split(n)` — equal division with fair remainder distribution
- `unchecked_plus()` / `unchecked_minus()` / `unchecked_mul()` — hot path variants
- `try_compare()` — checked comparison returning Result
- `get_currency_code_tiny()` / `get_currency_name_tiny()` — zero-copy TinyAsciiStr accessors
- `tick_power10_dp()` — now public

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
