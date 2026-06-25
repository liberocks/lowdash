use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_repeat_by(c: &mut Criterion) {
    c.bench_function("repeat_by/2048", |b| {
        b.iter(|| ld::repeat_by(black_box(2_048), black_box(|index| index as i32 * 2)))
    });

    c.bench_function("repeat_by/512", |b| {
        b.iter(|| ld::repeat_by(black_box(512), black_box(|index| index as f64 * 1.5)))
    });
}
