use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_replace_all(c: &mut Criterion) {
    let collection = support::duplicate_int_vec(4_096);
    c.bench_function("replace_all", |b| {
        b.iter(|| {
            ld::replace_all(black_box(&collection), black_box(7), black_box(999))
        })
    });
}
