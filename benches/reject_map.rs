use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_reject_map(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("reject_map", |b| {
        b.iter(|| {
            ld::reject_map(black_box(&collection), black_box(|value: &i32, index| (*value + index as i32, index % 2 == 0)))
        })
    });
}
