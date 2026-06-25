use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_pick_by_keys(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    let keys = vec![
        String::from("key-10"),
        String::from("key-20"),
        String::from("key-30"),
        String::from("key-40"),
    ];
    c.bench_function("pick_by_keys", |b| {
        b.iter(|| ld::pick_by_keys(black_box(&map), black_box(&keys)))
    });
}
