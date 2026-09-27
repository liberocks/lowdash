use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_mode(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).map(|value| value % 32).collect();
    c.bench_function("mode/large", |b| b.iter(|| ld::mode(black_box(&values))));

    let people = support::people(4_096);
    c.bench_function("mode/people", |b| b.iter(|| ld::mode(black_box(&people))));
}
