use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_times(c: &mut Criterion) {
    c.bench_function("times/2048", |b| {
        b.iter(|| ld::times(black_box(2_048), black_box(|index| index as i32 * 2)))
    });

    c.bench_function("times/128", |b| {
        b.iter(|| ld::times(black_box(128), black_box(|index| index as f64)))
    });
}
