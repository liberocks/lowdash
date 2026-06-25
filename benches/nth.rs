use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_nth(c: &mut Criterion) {
    let ints: Vec<i32> = (0..4_096).collect();
    c.bench_function("nth/int_vec/neg", |b| {
        b.iter(|| ld::nth(black_box(&ints), black_box(-10)))
    });

    c.bench_function("nth/int_vec/pos", |b| {
        b.iter(|| ld::nth(black_box(&ints), black_box(100)))
    });
}
