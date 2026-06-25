use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_compact(c: &mut Criterion) {
    let collection = support::defaulty_int_vec(4_096);
    c.bench_function("compact", |b| {
        b.iter(|| {
            ld::compact(black_box(&collection))
        })
    });
}
