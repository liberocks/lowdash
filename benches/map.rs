use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_map(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("map", |b| {
        b.iter(|| {
            ld::map(black_box(&collection), black_box(|value: &i32| *value * 2))
        })
    });
}
