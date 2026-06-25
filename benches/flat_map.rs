use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_flat_map(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("flat_map/int_vec", |b| {
        b.iter(|| {
            ld::flat_map(
                black_box(&ints),
                black_box(|value: &i32, _| vec![*value, *value * 2]),
            )
        })
    });

    let floats = support::float_vec(4_096);
    c.bench_function("flat_map/float_vec", |b| {
        b.iter(|| {
            ld::flat_map(
                black_box(&floats),
                black_box(|value: &f64, _| vec![*value, *value * 2.0]),
            )
        })
    });
}
