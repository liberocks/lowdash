use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_reduce_right(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("reduce_right", |b| {
        b.iter(|| {
            ld::reduce_right(black_box(&collection), black_box(|acc, value: &i32, _| acc + value), black_box(0_i32))
        })
    });
}
