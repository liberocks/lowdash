use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_shuffle(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("shuffle/int_vec", |b| b.iter(|| ld::shuffle(black_box(&ints))));

    let floats = support::float_vec(4_096);
    c.bench_function("shuffle/float_vec", |b| b.iter(|| ld::shuffle(black_box(&floats))));
}
