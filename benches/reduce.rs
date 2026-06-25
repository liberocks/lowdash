use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_reduce(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("reduce/int_vec", |b| {
        b.iter(|| {
            ld::reduce(
                black_box(&ints),
                black_box(|acc, value: &i32, _| acc + value),
                black_box(0_i32),
            )
        })
    });

    let floats = support::float_vec(4_096);
    c.bench_function("reduce/float_vec", |b| {
        b.iter(|| {
            ld::reduce(
                black_box(&floats),
                black_box(|acc, value: &f64, _| acc + value),
                black_box(0.0_f64),
            )
        })
    });
}
