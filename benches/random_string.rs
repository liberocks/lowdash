use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_random_string(c: &mut Criterion) {
    c.bench_function("random_string/64", |b| {
        b.iter(|| ld::random_string(black_box(64), black_box(ld::common::ALPHANUMERIC_CHARSET)))
    });

    c.bench_function("random_string/256", |b| {
        b.iter(|| ld::random_string(black_box(256), black_box(ld::common::ALPHANUMERIC_CHARSET)))
    });

    c.bench_function("random_string/8", |b| {
        b.iter(|| ld::random_string(black_box(8), black_box(ld::common::ALPHANUMERIC_CHARSET)))
    });
}
