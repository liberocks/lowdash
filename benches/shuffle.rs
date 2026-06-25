use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_shuffle(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("shuffle", |b| {
        b.iter(|| ld::shuffle(black_box(&collection)))
    });
}
