use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_find_duplicates_by(c: &mut Criterion) {
    let collection = support::people(2_048);
    c.bench_function("find_duplicates_by", |b| {
        b.iter(|| {
            ld::find_duplicates_by(black_box(&collection), black_box(|person: &support::Person| person.age))
        })
    });
}
