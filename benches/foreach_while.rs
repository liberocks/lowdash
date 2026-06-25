use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_foreach_while(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("foreach_while/int_vec", |b| {
        b.iter(|| {
            let mut total = 0_i64;
            ld::foreach_while(black_box(&ints), |value: &i32, _| {
                total += *value as i64;
                total < 10_000
            });
            black_box(total)
        })
    });

    let floats = support::float_vec(4_096);
    c.bench_function("foreach_while/float_vec", |b| {
        b.iter(|| {
            let mut total = 0.0_f64;
            ld::foreach_while(black_box(&floats), |value: &f64, _| {
                total += *value;
                total < 100.0
            });
            black_box(total)
        })
    });
}
