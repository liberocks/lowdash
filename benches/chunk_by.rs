use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_chunk_by(c: &mut Criterion) {
    let numbers = support::duplicate_int_vec(4_096);
    c.bench_function("chunk_by/int_vec", |b| {
        b.iter(|| ld::chunk_by(black_box(&numbers), black_box(|value: &i32| *value / 8)))
    });

    let people = support::people(4_096);
    c.bench_function("chunk_by/people", |b| {
        b.iter(|| {
            ld::chunk_by(
                black_box(&people),
                black_box(|person: &support::Person| person.age / 10),
            )
        })
    });
}
