use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_replace(c: &mut Criterion) {
    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("replace/duplicate_int_vec", |b| {
        b.iter(|| ld::replace(black_box(&duplicates), black_box(7), black_box(999), black_box(64)))
    });

    let ints = support::int_vec(4_096);
    c.bench_function("replace/int_vec", |b| {
        b.iter(|| ld::replace(black_box(&ints), black_box(7), black_box(999), black_box(64)))
    });
}
