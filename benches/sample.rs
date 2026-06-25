use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_sample(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("sample/int_vec", |b| b.iter(|| ld::sample(black_box(&ints))));

    let floats = support::float_vec(4_096);
    c.bench_function("sample/float_vec", |b| b.iter(|| ld::sample(black_box(&floats))));
}
