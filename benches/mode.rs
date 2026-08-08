use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_mode(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).map(|value| value % 32).collect();
    c.bench_function("mode/large", |b| b.iter(|| ld::mode(black_box(&values))));
}
