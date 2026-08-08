use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_scan(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    c.bench_function("scan/int_vec", |b| {
        b.iter(|| {
            ld::scan(
                black_box(&numbers),
                black_box(|sum, value: &i32, _| sum + value),
                black_box(0_i32),
            )
        })
    });

    let people = support::people(4_096);
    c.bench_function("scan/people_names", |b| {
        b.iter(|| {
            ld::scan(
                black_box(&people),
                black_box(|mut names: Vec<String>, person: &support::Person, _| {
                    names.push(person.name.clone());
                    names
                }),
                black_box(Vec::<String>::new()),
            )
        })
    });
}
