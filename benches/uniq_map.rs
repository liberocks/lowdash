use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_uniq_map(c: &mut Criterion) {
    let people = support::people(512);
    c.bench_function("uniq_map/people_ages", |b| {
        b.iter(|| ld::uniq_map(black_box(&people), |person, _| person.age))
    });

    let ints = support::int_vec(512);
    c.bench_function("uniq_map/indexed_values", |b| {
        b.iter(|| ld::uniq_map(black_box(&ints), |value, index| *value + index as i32))
    });
}
