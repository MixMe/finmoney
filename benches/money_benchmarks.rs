//! Benchmarks for the finmoney library.

use criterion::{Criterion, criterion_group, criterion_main};
use finmoney::{FinMoney, FinMoneyCurrency, MoneyRoundingStrategy};
use rust_decimal_macros::dec;
use std::hint::black_box;

fn benchmark_fin_money_creation(c: &mut Criterion) {
    let usd = FinMoneyCurrency::USD;

    c.bench_function("FinMoney_creation", |b| {
        b.iter(|| FinMoney::new(black_box(dec!(10.50)), black_box(usd)))
    });
}

fn benchmark_fin_money_arithmetic(c: &mut Criterion) {
    let usd = FinMoneyCurrency::USD;
    let fin_money1 = FinMoney::new(dec!(10.50), usd);
    let fin_money2 = FinMoney::new(dec!(5.25), usd);

    c.bench_function("FinMoney_addition", |b| {
        b.iter(|| (black_box(fin_money1) + black_box(fin_money2)).unwrap())
    });

    c.bench_function("FinMoney_subtraction", |b| {
        b.iter(|| (black_box(fin_money1) - black_box(fin_money2)).unwrap())
    });

    c.bench_function("FinMoney_multiplication", |b| {
        b.iter(|| black_box(fin_money1) * black_box(dec!(2)))
    });

    c.bench_function("FinMoney_division", |b| {
        b.iter(|| {
            fin_money1
                .divided_by_decimal(
                    black_box(dec!(2)),
                    black_box(MoneyRoundingStrategy::MidpointNearestEven),
                )
                .unwrap()
        })
    });
}

fn benchmark_fin_money_comparisons(c: &mut Criterion) {
    let usd = FinMoneyCurrency::USD;
    let fin_money1 = FinMoney::new(dec!(10.50), usd);
    let fin_money2 = FinMoney::new(dec!(5.25), usd);

    c.bench_function("FinMoney_comparison", |b| {
        b.iter(|| {
            black_box(fin_money1)
                .is_greater_than(black_box(fin_money2))
                .unwrap()
        })
    });

    c.bench_function("FinMoney_min_max", |b| {
        b.iter(|| {
            let min = black_box(fin_money1).min(black_box(fin_money2)).unwrap();
            let max = black_box(fin_money1).max(black_box(fin_money2)).unwrap();
            (min, max)
        })
    });
}

fn benchmark_tick_operations(c: &mut Criterion) {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.567), usd);

    c.bench_function("tick_rounding_power_of_ten", |b| {
        b.iter(|| {
            black_box(fin_money)
                .to_tick_nearest(black_box(dec!(0.01)))
                .unwrap()
        })
    });

    c.bench_function("tick_rounding_general", |b| {
        b.iter(|| {
            black_box(fin_money)
                .to_tick_nearest(black_box(dec!(0.25)))
                .unwrap()
        })
    });

    c.bench_function("tick_validation", |b| {
        b.iter(|| black_box(fin_money).is_multiple_of_tick(black_box(dec!(0.25))))
    });
}

fn benchmark_percentage_calculations(c: &mut Criterion) {
    let usd = FinMoneyCurrency::USD;
    let initial = FinMoney::new(dec!(100), usd);
    let current = FinMoney::new(dec!(110), usd);

    c.bench_function("percentage_change", |b| {
        b.iter(|| {
            black_box(current)
                .percent_change_from(black_box(initial))
                .unwrap()
        })
    });
}

fn benchmark_rounding_strategies(c: &mut Criterion) {
    let usd = FinMoneyCurrency::USD;
    let fin_money = FinMoney::new(dec!(10.555), usd);

    c.bench_function("rounding_nearest_even", |b| {
        b.iter(|| {
            black_box(fin_money).round_dp_with_strategy(
                black_box(2),
                black_box(MoneyRoundingStrategy::MidpointNearestEven),
            )
        })
    });

    c.bench_function("rounding_away_from_zero", |b| {
        b.iter(|| {
            black_box(fin_money).round_dp_with_strategy(
                black_box(2),
                black_box(MoneyRoundingStrategy::MidpointAwayFromZero),
            )
        })
    });
}

fn benchmark_currency_operations(c: &mut Criterion) {
    c.bench_function("currency_creation", |b| {
        b.iter(|| {
            FinMoneyCurrency::new(
                black_box(1),
                black_box("USD".to_string()),
                black_box(Some("US Dollar".to_string())),
                black_box(2),
            )
            .unwrap()
        })
    });

    c.bench_function("currency_sanitized_creation", |b| {
        b.iter(|| {
            FinMoneyCurrency::new_sanitized(
                black_box(1),
                black_box("USD".to_string()),
                black_box(Some("US Dollar".to_string())),
                black_box(2),
            )
        })
    });
}

criterion_group!(
    benches,
    benchmark_fin_money_creation,
    benchmark_fin_money_arithmetic,
    benchmark_fin_money_comparisons,
    benchmark_tick_operations,
    benchmark_percentage_calculations,
    benchmark_rounding_strategies,
    benchmark_currency_operations
);

criterion_main!(benches);
