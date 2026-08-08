use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_median_low(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).rev().collect();
    c.bench_function("median_low/large", |b| {
        b.iter(|| ld::median_low(black_box(&values)))
    });
}
