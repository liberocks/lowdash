use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_earliest(c: &mut Criterion) {
    let collection = support::time_vec(4_096);
    c.bench_function("earliest", |b| {
        b.iter(|| ld::earliest(black_box(&collection)))
    });
}
