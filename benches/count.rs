use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_count(c: &mut Criterion) {
    let collection = support::duplicate_int_vec(4_096);
    c.bench_function("count", |b| {
        b.iter(|| {
            ld::count(black_box(&collection), black_box(7))
        })
    });
}
