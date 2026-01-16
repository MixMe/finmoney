//! Basic usage examples for the finmoney library.

use finmoney::{FinMoney, FinMoneyCurrency, FinMoneyRoundingStrategy};
use rust_decimal_macros::dec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== finmoney Basic Usage Examples ===\n");

    // Create currencies
    let usd = FinMoneyCurrency::new(1, "USD".to_string(), Some("US Dollar".to_string()), 2)?;
    let eur = FinMoneyCurrency::new(2, "EUR".to_string(), Some("Euro".to_string()), 2)?;

    // Or use predefined currencies
    let btc = FinMoneyCurrency::BTC;

    println!("1. Creating FinMoney Values");
    let price = FinMoney::new(dec!(10.50), usd);
    let tax = FinMoney::new(dec!(1.05), usd);
    let btc_amount = FinMoney::new(dec!(0.00123456), btc);

    println!("Price: {}", price);
    println!("Tax: {}", tax);
    println!("BTC Amount: {}", btc_amount);
    println!();

    println!("2. Basic Arithmetic");
    let total = (price + tax)?;
    println!("Price + Tax = {}", total);

    let doubled = price * dec!(2);
    println!("Price * 2 = {}", doubled);

    let divided =
        price.divided_by_decimal(dec!(3), FinMoneyRoundingStrategy::MidpointNearestEven)?;
    println!("Price / 3 = {}", divided);
    println!();

    println!("3. Currency Safety");
    let eur_amount = FinMoney::new(dec!(85.50), eur);
    match price + eur_amount {
        Ok(result) => println!("USD + EUR = {}", result),
        Err(e) => println!("Error mixing currencies: {}", e),
    }
    println!();

    println!("4. Comparisons");
    let price2 = FinMoney::new(dec!(9.75), usd);

    if price.is_greater_than(price2)? {
        println!("{} is greater than {}", price, price2);
    }

    let min_price: FinMoney = price.min(price2)?;
    let max_price: FinMoney = price.max(price2)?;
    println!("Min price: {}", min_price);
    println!("Max price: {}", max_price);
    println!();

    println!("5. Properties");
    let negative_amount = FinMoney::new(dec!(-15.75), usd);
    println!("Amount: {}", negative_amount);
    println!("Is zero: {}", negative_amount.is_zero());
    println!("Is positive: {}", negative_amount.is_positive());
    println!("Is negative: {}", negative_amount.is_negative());
    println!("Has fraction: {}", negative_amount.has_fraction());
    println!("Absolute value: {}", negative_amount.abs());
    println!();

    println!("6. Percentage Calculations");
    let initial = FinMoney::new(dec!(100), usd);
    let current = FinMoney::new(dec!(110), usd);
    let change = current.percent_change_from(initial)?;
    println!("Change from {} to {}: {}%", initial, current, change);
    println!();

    println!("7. Rounding");
    let precise_amount = FinMoney::new(dec!(10.567), usd);
    println!("Original: {}", precise_amount);

    let rounded_even =
        precise_amount.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointNearestEven);
    let rounded_away =
        precise_amount.round_dp_with_strategy(2, FinMoneyRoundingStrategy::MidpointAwayFromZero);

    println!("Rounded (nearest even): {}", rounded_even);
    println!("Rounded (away from zero): {}", rounded_away);

    Ok(())
}
