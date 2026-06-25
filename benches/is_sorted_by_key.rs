use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_is_sorted_by_key(c: &mut Criterion) {
    let collection: Vec<support::Person> = (0..512).map(|i| support::Person { id: i, name: format!("person-{i}"), age: i as u32 }).collect();
    c.bench_function("is_sorted_by_key", |b| {
        b.iter(|| {
            ld::is_sorted_by_key(black_box(&collection), black_box(|person: &support::Person| person.age))
        })
    });
}
