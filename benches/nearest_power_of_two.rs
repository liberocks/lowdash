use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_nearest_power_of_two(c: &mut Criterion) {
    c.bench_function("nearest_power_of_two/65537", |b| {
        b.iter(|| ld::nearest_power_of_two(black_box(65_537)))
    });

    c.bench_function("nearest_power_of_two/1", |b| {
        b.iter(|| ld::nearest_power_of_two(black_box(1)))
    });

    c.bench_function("nearest_power_of_two/1000", |b| {
        b.iter(|| ld::nearest_power_of_two(black_box(1000)))
    });
}
