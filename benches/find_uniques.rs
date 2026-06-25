use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_find_uniques(c: &mut Criterion) {
    let collection = support::duplicate_int_vec(4_096);
    c.bench_function("find_uniques", |b| {
        b.iter(|| {
            ld::find_uniques(black_box(&collection))
        })
    });
}
