use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_union_by(c: &mut Criterion) {
    let collections = vec![
        support::duplicate_int_vec(1_024),
        support::duplicate_int_vec(1_024),
        support::duplicate_int_vec(1_024),
    ];
    c.bench_function("union_by/int_vec", |b| {
        b.iter(|| ld::union_by(black_box(&collections), black_box(|value: &i32| *value)))
    });

    let people = vec![
        support::people(512),
        support::people(512),
        support::people(512),
    ];
    c.bench_function("union_by/people", |b| {
        b.iter(|| {
            ld::union_by(
                black_box(&people),
                black_box(|person: &support::Person| person.age),
            )
        })
    });
}
