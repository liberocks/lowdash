use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_reverse(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("reverse", |b| {
        b.iter(|| {
            ld::reverse(black_box(&collection))
        })
    });
}
