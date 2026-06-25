use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_nearest_power_of_two(c: &mut Criterion) {
    c.bench_function("nearest_power_of_two", |b| {
        b.iter(|| {
            ld::nearest_power_of_two(black_box(65_537))
        })
    });
}
