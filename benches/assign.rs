use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_assign(c: &mut Criterion) {
    let small = support::numeric_maps(8, 256);
    c.bench_function("assign/small", |b| b.iter(|| ld::assign(black_box(&small))));

    let large = support::numeric_maps(32, 1024);
    c.bench_function("assign/large", |b| b.iter(|| ld::assign(black_box(&large))));
}
