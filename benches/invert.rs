use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_invert(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("invert/medium", |b| b.iter(|| ld::invert(black_box(&map))));

    let small = support::numeric_map(64);
    c.bench_function("invert/small", |b| b.iter(|| ld::invert(black_box(&small))));
}
