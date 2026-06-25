use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_pick_by_keys(c: &mut Criterion) {
    let keys = vec![
        String::from("key-10"), String::from("key-20"),
        String::from("key-30"), String::from("key-40"),
    ];

    let map = support::numeric_map(2_048);
    c.bench_function("pick_by_keys/medium", |b| {
        b.iter(|| ld::pick_by_keys(black_box(&map), black_box(&keys)))
    });

    let small = support::numeric_map(64);
    c.bench_function("pick_by_keys/small", |b| {
        b.iter(|| ld::pick_by_keys(black_box(&small), black_box(&keys)))
    });
}
