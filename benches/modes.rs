use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_modes(c: &mut Criterion) {
    let numbers = support::duplicate_int_vec(4_096);
    c.bench_function("modes/int_vec", |b| {
        b.iter(|| ld::modes(black_box(&numbers)))
    });

    let people = support::people(4_096);
    c.bench_function("modes/people", |b| b.iter(|| ld::modes(black_box(&people))));
}
