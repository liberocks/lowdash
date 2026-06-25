use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_foreach_while(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("foreach_while", |b| {
        b.iter(|| {
            let mut total = 0_i64;
            ld::foreach_while(black_box(&collection), |value: &i32, _| {
                total += *value as i64;
                total < 10_000
            });
            black_box(total)
        })
    });
}
