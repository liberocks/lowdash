use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_factorial(c: &mut Criterion) {
    c.bench_function("factorial/34", |b| b.iter(|| ld::factorial(black_box(34))));
}
