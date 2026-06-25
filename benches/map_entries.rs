use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_map_entries(c: &mut Criterion) {
    let map = support::numeric_map(1_024);
    c.bench_function("map_entries", |b| {
        b.iter(|| {
            ld::map_entries(black_box(&map), black_box(|key: &String, value: &i32| (format!("mapped-{key}"), *value * 2)))
        })
    });
}
