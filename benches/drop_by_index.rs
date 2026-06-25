use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_drop_by_index(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    let indexes = vec![1, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    c.bench_function("drop_by_index", |b| {
        b.iter(|| {
            ld::drop_by_index(black_box(&collection), black_box(&indexes))
        })
    });
}
