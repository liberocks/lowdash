use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_drop_right(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("drop_right", |b| {
        b.iter(|| ld::drop_right(black_box(&collection), black_box(256)))
    });
}
