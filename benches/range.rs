use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_range(c: &mut Criterion) {
    c.bench_function("range/10000", |b| b.iter(|| ld::range(black_box(10_000))));

    c.bench_function("range/100", |b| b.iter(|| ld::range(black_box(100))));
}
