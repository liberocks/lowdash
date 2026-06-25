use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_is_sorted(c: &mut Criterion) {
    let collection: Vec<i32> = (0..4_096).collect();
    c.bench_function("is_sorted", |b| {
        b.iter(|| {
            ld::is_sorted(black_box(&collection))
        })
    });
}
