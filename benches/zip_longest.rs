use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_zip_longest(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    let people = support::people(4_096);
    c.bench_function("zip_longest/equal_lengths", |b| {
        b.iter(|| ld::zip_longest(black_box(&numbers), black_box(&people)))
    });

    let short_numbers = support::int_vec(512);
    c.bench_function("zip_longest/short_left", |b| {
        b.iter(|| ld::zip_longest(black_box(&short_numbers), black_box(&people)))
    });
}
