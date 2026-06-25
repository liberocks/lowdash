use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_map_to_slice(c: &mut Criterion) {
    let map = support::numeric_map(1_024);
    c.bench_function("map_to_slice", |b| {
        b.iter(|| {
            ld::map_to_slice(
                black_box(&map),
                black_box(|key: &String, value: &i32| format!("{key}:{value}")),
            )
        })
    });
}
