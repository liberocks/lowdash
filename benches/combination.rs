use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_combination(c: &mut Criterion) {
    let items: Vec<i32> = (0..18).collect();
    c.bench_function("combination/18-choose-3", |b| {
        b.iter(|| ld::combination(black_box(&items), black_box(3)))
    });

    c.bench_function("combination/18-choose-2", |b| {
        b.iter(|| ld::combination(black_box(&items), black_box(2)))
    });

    let small: Vec<i32> = (0..8).collect();
    c.bench_function("combination/8-choose-3", |b| {
        b.iter(|| ld::combination(black_box(&small), black_box(3)))
    });
}
