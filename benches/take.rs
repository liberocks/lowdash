use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_take(c: &mut Criterion) {
    let small: Vec<i32> = (0..128).collect();
    c.bench_function("take/small", |b| {
        b.iter(|| ld::take(black_box(&small), black_box(32)))
    });

    let large: Vec<i32> = (0..4_096).collect();
    c.bench_function("take/large", |b| {
        b.iter(|| ld::take(black_box(&large), black_box(2_048)))
    });

    c.bench_function("take/oversized", |b| {
        b.iter(|| ld::take(black_box(&large), black_box(usize::MAX)))
    });
}
