use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_max_by_key(c: &mut Criterion) {
    let people = support::people(4_096);
    c.bench_function("max_by_key/people", |b| {
        b.iter(|| {
            ld::max_by_key(
                black_box(&people),
                black_box(|person: &support::Person| person.age),
            )
        })
    });

    let descending = support::people_descending(4_096);
    c.bench_function("max_by_key/people_descending", |b| {
        b.iter(|| {
            ld::max_by_key(
                black_box(&descending),
                black_box(|person: &support::Person| person.age),
            )
        })
    });
}
