use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_find_key(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("find_key/map/exists", |b| {
        b.iter(|| ld::find_key(black_box(&map), black_box(13)))
    });

    c.bench_function("find_key/map/missing", |b| {
        b.iter(|| ld::find_key(black_box(&map), black_box(999)))
    });
}
