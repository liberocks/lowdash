use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_take_right(c: &mut Criterion) {
    let small: Vec<i32> = (0..128).collect();
    c.bench_function("take_right/small", |b| {
        b.iter(|| ld::take_right(black_box(&small), black_box(32)))
    });

    let large: Vec<i32> = (0..4_096).collect();
    c.bench_function("take_right/large", |b| {
        b.iter(|| ld::take_right(black_box(&large), black_box(2_048)))
    });

    c.bench_function("take_right/oversized", |b| {
        b.iter(|| ld::take_right(black_box(&large), black_box(usize::MAX)))
    });
}
