use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_has_key(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    let key = String::from("key-1024");
    c.bench_function("has_key", |b| {
        b.iter(|| ld::has_key(black_box(&map), black_box(&key)))
    });
}
