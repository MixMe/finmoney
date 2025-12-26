//! Trading and tick handling examples for the finmoney library.

use finmoney::{FinMoney, FinMoneyCurrency};
use rust_decimal_macros::dec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== finmoney Trading & Tick Examples ===\n");

    let usd = FinMoneyCurrency::USD;

    println!("1. Basic Tick Rounding");
    let price = FinMoney::new(dec!(10.567), usd);
    println!("Original price: {}", price);

    // Round to different tick sizes
    let tick_25 = price.to_tick_nearest(dec!(0.25))?;
    let tick_10 = price.to_tick_nearest(dec!(0.10))?;
    let tick_01 = price.to_tick_nearest(dec!(0.01))?;

    println!("Rounded to 0.25 tick: {}", tick_25);
    println!("Rounded to 0.10 tick: {}", tick_10);
    println!("Rounded to 0.01 tick: {}", tick_01);
    println!();

    println!("2. Directional Tick Rounding");
    let price = FinMoney::new(dec!(10.567), usd);
    println!("Original price: {}", price);

    let tick_down = price.to_tick_down(dec!(0.25))?;
    let tick_up = price.to_tick_up(dec!(0.25))?;
    let tick_nearest = price.to_tick_nearest(dec!(0.25))?;

    println!("Tick down (floor): {}", tick_down);
    println!("Tick up (ceil): {}", tick_up);
    println!("Tick nearest: {}", tick_nearest);
    println!();

    println!("3. Tick Validation");
    let valid_price = FinMoney::new(dec!(10.50), usd);
    let invalid_price = FinMoney::new(dec!(10.567), usd);

    println!(
        "Price {}: valid for 0.25 tick? {}",
        valid_price,
        valid_price.is_multiple_of_tick(dec!(0.25))
    );
    println!(
        "Price {}: valid for 0.25 tick? {}",
        invalid_price,
        invalid_price.is_multiple_of_tick(dec!(0.25))
    );
    println!(
        "Price {}: valid for 0.01 tick? {}",
        invalid_price,
        invalid_price.is_multiple_of_tick(dec!(0.01))
    );
    println!();

    println!("4. Exchange-Style Order Book");
    simulate_order_book()?;
    println!();

    println!("5. Crypto Trading Example");
    simulate_crypto_trading()?;

    Ok(())
}

fn simulate_order_book() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Simulating Order Book with 0.25 Tick Size ---");

    let usd = FinMoneyCurrency::USD;
    let tick_size = dec!(0.25);

    // Incoming orders with arbitrary prices
    let orders = vec![
        ("Buy Order 1", dec!(99.87)),
        ("Buy Order 2", dec!(99.63)),
        ("Sell Order 1", dec!(100.12)),
        ("Sell Order 2", dec!(100.38)),
    ];

    for (order_name, price) in orders {
        let original = FinMoney::new(price, usd);
        let rounded = original.to_tick_nearest(tick_size)?;

        println!(
            "{}: {} -> {} ({})",
            order_name,
            original,
            rounded,
            if original.is_multiple_of_tick(tick_size) {
                "valid"
            } else {
                "adjusted"
            }
        );
    }

    Ok(())
}

fn simulate_crypto_trading() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Crypto Trading with Different Tick Sizes ---");

    let btc = FinMoneyCurrency::BTC;
    let usd = FinMoneyCurrency::USD;

    // BTC price in USD with 0.01 tick
    let btc_price = FinMoney::new(dec!(43567.89), usd);
    let btc_tick = dec!(0.01);
    let rounded_btc_price = btc_price.to_tick_nearest(btc_tick)?;

    println!("BTC Price: {} -> {}", btc_price, rounded_btc_price);

    // BTC quantity with 8 decimal places (satoshi precision)
    let btc_quantity = FinMoney::new(dec!(0.12345678), btc);
    let satoshi_tick = dec!(0.00000001); // 1 satoshi
    let rounded_quantity = btc_quantity.to_tick_nearest(satoshi_tick)?;

    println!("BTC Quantity: {} -> {}", btc_quantity, rounded_quantity);

    // Calculate total value
    let total_usd = rounded_btc_price.multiplied_by_decimal(rounded_quantity.get_amount());
    let final_total = total_usd.to_tick_nearest(dec!(0.01))?;

    println!("Total Value: {}", final_total);

    // Demonstrate different rounding strategies for fees
    let fee_rate = dec!(0.001); // 0.1% fee
    let raw_fee = final_total.multiplied_by_decimal(fee_rate);

    println!("\nFee Calculations:");
    println!("Raw fee: {}", raw_fee);

    let fee_rounded_down = raw_fee.to_tick_down(dec!(0.01))?;
    let fee_rounded_up = raw_fee.to_tick_up(dec!(0.01))?;

    println!("Fee (rounded down): {}", fee_rounded_down);
    println!("Fee (rounded up): {}", fee_rounded_up);

    Ok(())
}
