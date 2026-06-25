use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_count_values(c: &mut Criterion) {
    let collection = support::duplicate_int_vec(4_096);
    c.bench_function("count_values", |b| {
        b.iter(|| {
            ld::count_values(black_box(&collection))
        })
    });
}
