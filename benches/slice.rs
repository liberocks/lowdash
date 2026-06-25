use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_slice(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("slice/int_vec/neg", |b| {
        b.iter(|| ld::slice(black_box(&ints), black_box(-1_024), black_box(-128)))
    });

    c.bench_function("slice/int_vec/pos", |b| {
        b.iter(|| ld::slice(black_box(&ints), black_box(128), black_box(512)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("slice/float_vec/neg", |b| {
        b.iter(|| ld::slice(black_box(&floats), black_box(-1_024), black_box(-128)))
    });
}
