use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_foreach(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("foreach/int_vec", |b| {
        b.iter(|| {
            let mut total = 0_i64;
            ld::foreach(black_box(&ints), |value: &i32, _| total += *value as i64);
            black_box(total)
        })
    });

    let floats = support::float_vec(4_096);
    c.bench_function("foreach/float_vec", |b| {
        b.iter(|| {
            let mut total = 0.0_f64;
            ld::foreach(black_box(&floats), |value: &f64, _| total += *value);
            black_box(total)
        })
    });
}
