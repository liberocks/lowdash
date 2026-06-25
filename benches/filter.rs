use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_filter(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("filter", |b| {
        b.iter(|| {
            ld::filter(
                black_box(&collection),
                black_box(|value: &i32, _| *value % 2 == 0),
            )
        })
    });
}
