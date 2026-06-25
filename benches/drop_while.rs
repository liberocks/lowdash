use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_drop_while(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("drop_while", |b| {
        b.iter(|| ld::drop_while(black_box(&collection), black_box(|value: &i32| *value < 10)))
    });
}
