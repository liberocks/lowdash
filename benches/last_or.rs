use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_last_or(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("last_or", |b| {
        b.iter(|| ld::last_or(black_box(&collection), black_box(-1)))
    });
}
