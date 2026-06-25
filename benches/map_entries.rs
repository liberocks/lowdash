use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_map_entries(c: &mut Criterion) {
    let map = support::numeric_map(1_024);
    c.bench_function("map_entries/medium", |b| {
        b.iter(|| {
            ld::map_entries(
                black_box(&map),
                black_box(|key: &String, value: &i32| (format!("mapped-{key}"), *value * 2)),
            )
        })
    });

    let small = support::numeric_map(64);
    c.bench_function("map_entries/small", |b| {
        b.iter(|| {
            ld::map_entries(
                black_box(&small),
                black_box(|key: &String, value: &i32| (format!("mapped-{key}"), *value * 2)),
            )
        })
    });
}
