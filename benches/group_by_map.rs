use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_group_by_map(c: &mut Criterion) {
    let people = support::people(4_096);
    c.bench_function("group_by_map/people", |b| {
        b.iter(|| ld::group_by_map(black_box(&people), |person| (person.age, person.id)))
    });

    let ints = support::int_vec(4_096);
    c.bench_function("group_by_map/int_vec", |b| {
        b.iter(|| ld::group_by_map(black_box(&ints), |value| (*value % 2, *value)))
    });
}
