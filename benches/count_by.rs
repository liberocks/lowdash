use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_count_by(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("count_by", |b| {
        b.iter(|| ld::count_by(black_box(&collection), black_box(|value: &i32| *value > 0)))
    });
}
