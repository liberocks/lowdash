use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_range(c: &mut Criterion) {
    c.bench_function("range/10000", |b| b.iter(|| ld::range(black_box(10_000))));

    c.bench_function("range/100", |b| b.iter(|| ld::range(black_box(100))));
}
