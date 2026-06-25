use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_chunk(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("chunk", |b| {
        b.iter(|| ld::chunk(black_box(&collection), black_box(32)))
    });
}
