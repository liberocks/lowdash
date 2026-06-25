use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_filter_map(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("filter_map/int_vec", |b| {
        b.iter(|| {
            ld::filter_map(
                black_box(&ints),
                black_box(|value: &i32, index| (*value + index as i32, index % 2 == 0)),
            )
        })
    });

    let floats = support::float_vec(4_096);
    c.bench_function("filter_map/float_vec", |b| {
        b.iter(|| {
            ld::filter_map(
                black_box(&floats),
                black_box(|value: &f64, index| (*value + index as f64, index % 2 == 0)),
            )
        })
    });
}
