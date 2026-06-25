use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_foreach(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("foreach", |b| {
        b.iter(|| {
            let mut total = 0_i64;
            ld::foreach(black_box(&collection), |value: &i32, _| {
                total += *value as i64
            });
            black_box(total)
        })
    });
}
