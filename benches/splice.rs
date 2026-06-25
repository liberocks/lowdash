use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_splice(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    let elements = vec![7, 8, 9, 10];
    c.bench_function("splice", |b| {
        b.iter(|| {
            ld::splice(black_box(&collection), black_box(128), black_box(&elements))
        })
    });
}
