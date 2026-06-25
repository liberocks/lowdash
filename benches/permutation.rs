use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_permutation(c: &mut Criterion) {
    let items = vec![1, 2, 3, 4, 5, 6, 7];
    c.bench_function("permutation/7", |b| {
        b.iter(|| ld::permutation(black_box(&items), black_box(7)))
    });

    let small = vec![1, 2, 3, 4];
    c.bench_function("permutation/4", |b| {
        b.iter(|| ld::permutation(black_box(&small), black_box(4)))
    });
}
