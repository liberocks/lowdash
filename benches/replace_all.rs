use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_replace_all(c: &mut Criterion) {
    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("replace_all/duplicate_int_vec", |b| {
        b.iter(|| ld::replace_all(black_box(&duplicates), black_box(7), black_box(999)))
    });

    let ints = support::int_vec(4_096);
    c.bench_function("replace_all/int_vec", |b| {
        b.iter(|| ld::replace_all(black_box(&ints), black_box(7), black_box(999)))
    });
}
