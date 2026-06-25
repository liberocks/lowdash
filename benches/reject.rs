use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_reject(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("reject", |b| {
        b.iter(|| {
            ld::reject(black_box(&collection), black_box(|value: &i32| *value % 2 == 0))
        })
    });
}
