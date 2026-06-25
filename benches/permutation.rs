use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_permutation(c: &mut Criterion) {
    let items = vec![1, 2, 3, 4, 5, 6, 7];
    c.bench_function("permutation", |b| {
        b.iter(|| ld::permutation(black_box(&items), 7))
    });
}
