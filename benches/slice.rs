use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_slice(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("slice", |b| {
        b.iter(|| {
            ld::slice(black_box(&collection), black_box(-1_024), black_box(-128))
        })
    });
}
