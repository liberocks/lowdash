use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_drop(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("drop", |b| {
        b.iter(|| ld::drop(black_box(&collection), black_box(256)))
    });
}
