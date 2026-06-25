use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_uniq(c: &mut Criterion) {
    let collection = support::duplicate_int_vec(4_096);
    c.bench_function("uniq", |b| {
        b.iter(|| {
            ld::uniq(black_box(&collection))
        })
    });
}
