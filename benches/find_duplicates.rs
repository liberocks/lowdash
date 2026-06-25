use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_find_duplicates(c: &mut Criterion) {
    let collection = support::duplicate_int_vec(4_096);
    c.bench_function("find_duplicates", |b| {
        b.iter(|| ld::find_duplicates(black_box(&collection)))
    });
}
