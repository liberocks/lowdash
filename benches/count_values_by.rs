use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_count_values_by(c: &mut Criterion) {
    let collection = support::people(2_048);
    c.bench_function("count_values_by", |b| {
        b.iter(|| {
            ld::count_values_by(black_box(&collection), black_box(|person: &support::Person| person.age / 10))
        })
    });
}
