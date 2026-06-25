use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_map_keys(c: &mut Criterion) {
    let map = support::numeric_map(1_024);
    c.bench_function("map_keys/medium", |b| {
        b.iter(|| {
            ld::map_keys(
                black_box(&map),
                black_box(|value: &i32, key: &String| format!("{key}-{}", value.abs())),
            )
        })
    });

    let small = support::numeric_map(64);
    c.bench_function("map_keys/small", |b| {
        b.iter(|| {
            ld::map_keys(
                black_box(&small),
                black_box(|value: &i32, key: &String| format!("{key}-{}", value.abs())),
            )
        })
    });
}
