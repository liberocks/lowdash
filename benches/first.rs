use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_first(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("first/int_vec", |b| b.iter(|| ld::first(black_box(&ints))));

    let floats = support::float_vec(4_096);
    c.bench_function("first/float_vec", |b| b.iter(|| ld::first(black_box(&floats))));
}
