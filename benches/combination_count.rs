use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_combination_count(c: &mut Criterion) {
    c.bench_function("combination_count/52-choose-5", |b| {
        b.iter(|| ld::combination_count(black_box(52), black_box(5)))
    });
}
