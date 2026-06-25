use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_repeat(c: &mut Criterion) {
    c.bench_function("repeat/2048/7", |b| {
        b.iter(|| ld::repeat(black_box(2_048), black_box(7_i32)))
    });

    c.bench_function("repeat/512/3.14", |b| {
        b.iter(|| ld::repeat(black_box(512), black_box(3.14_f64)))
    });
}
