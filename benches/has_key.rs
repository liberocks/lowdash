use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_has_key(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    let existing = String::from("key-1024");
    c.bench_function("has_key/map/exists", |b| {
        b.iter(|| ld::has_key(black_box(&map), black_box(&existing)))
    });

    let missing = String::from("missing");
    c.bench_function("has_key/map/missing", |b| {
        b.iter(|| ld::has_key(black_box(&map), black_box(&missing)))
    });
}
