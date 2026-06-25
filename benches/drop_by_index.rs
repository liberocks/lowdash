use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_drop_by_index(c: &mut Criterion) {
    let indexes = vec![1, 3, 5, 8, 13, 21, 34, 55, 89, 144];

    let ints = support::int_vec(4_096);
    c.bench_function("drop_by_index/int_vec", |b| {
        b.iter(|| ld::drop_by_index(black_box(&ints), black_box(&indexes)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("drop_by_index/float_vec", |b| {
        b.iter(|| ld::drop_by_index(black_box(&floats), black_box(&indexes)))
    });

    let more_indexes: Vec<isize> = (0..256).step_by(3).collect();
    c.bench_function("drop_by_index/int_vec/256", |b| {
        b.iter(|| ld::drop_by_index(black_box(&ints), black_box(&more_indexes)))
    });
}
