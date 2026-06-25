use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_find(c: &mut Criterion) {
    let ints: Vec<i32> = (0..4_096).collect();
    c.bench_function("find/int_vec/exists", |b| {
        b.iter(|| ld::find(black_box(&ints), black_box(|value: &i32| *value == 4_095)))
    });

    c.bench_function("find/int_vec/missing", |b| {
        b.iter(|| ld::find(black_box(&ints), black_box(|value: &i32| *value == -1)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("find/float_vec", |b| {
        b.iter(|| ld::find(black_box(&floats), black_box(|value: &f64| *value > 10.0)))
    });
}
