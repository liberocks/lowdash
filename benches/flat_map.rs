use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_flat_map(c: &mut Criterion) {
    let collection = support::int_vec(2_048);
    c.bench_function("flat_map", |b| {
        b.iter(|| {
            ld::flat_map(
                black_box(&collection),
                black_box(|value: &i32, _| vec![*value, *value * 2]),
            )
        })
    });
}
