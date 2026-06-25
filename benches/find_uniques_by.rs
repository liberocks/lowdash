use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_find_uniques_by(c: &mut Criterion) {
    let collection = support::people(2_048);
    c.bench_function("find_uniques_by", |b| {
        b.iter(|| {
            ld::find_uniques_by(black_box(&collection), black_box(|person: &support::Person| person.age))
        })
    });
}
