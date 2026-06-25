use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_replace(c: &mut Criterion) {
    let collection = support::duplicate_int_vec(4_096);
    c.bench_function("replace", |b| {
        b.iter(|| {
            ld::replace(
                black_box(&collection),
                black_box(7),
                black_box(999),
                black_box(64),
            )
        })
    });
}
