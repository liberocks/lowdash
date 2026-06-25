use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_value_or(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    let key = String::from("missing");
    c.bench_function("value_or", |b| {
        b.iter(|| {
            ld::value_or(black_box(&map), black_box(&key), black_box(-1))
        })
    });
}
