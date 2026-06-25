use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_clamp(c: &mut Criterion) {
    c.bench_function("clamp", |b| {
        b.iter(|| ld::clamp(black_box(123_i32), black_box(-10_i32), black_box(90_i32)))
    });
}
