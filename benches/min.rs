use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_min(c: &mut Criterion) {
    let floats = support::float_vec(4_096);
    c.bench_function("min/float_vec", |b| b.iter(|| ld::min(black_box(&floats))));

    let ints = support::int_vec(4_096);
    c.bench_function("min/int_vec", |b| b.iter(|| ld::min(black_box(&ints))));
}
