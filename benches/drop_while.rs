use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_drop_while(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("drop_while/int_vec", |b| {
        b.iter(|| ld::drop_while(black_box(&ints), black_box(|value: &i32| *value < 10)))
    });

    let descending = support::int_vec_descending(4_096);
    c.bench_function("drop_while/descending", |b| {
        b.iter(|| ld::drop_while(black_box(&descending), black_box(|value: &i32| *value < 10)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("drop_while/float_vec", |b| {
        b.iter(|| ld::drop_while(black_box(&floats), black_box(|value: &f64| *value < 10.0)))
    });
}
