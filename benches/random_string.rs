use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_random_string(c: &mut Criterion) {
    c.bench_function("random_string", |b| {
        b.iter(|| ld::random_string(black_box(64), black_box(ld::common::ALPHANUMERIC_CHARSET)))
    });
}
