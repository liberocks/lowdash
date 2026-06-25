use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_map_values(c: &mut Criterion) {
    let map = support::numeric_map(1_024);
    c.bench_function("map_values", |b| {
        b.iter(|| {
            ld::map_values(black_box(&map), black_box(|value: &i32, key: &String| format!("{key}:{value}")))
        })
    });
}
