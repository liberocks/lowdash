use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_unzip(c: &mut Criterion) {
    let pairs: Vec<(i32, support::Person)> = support::int_vec(4_096)
        .into_iter()
        .zip(support::people(4_096))
        .collect();
    c.bench_function("unzip/int_person_pairs", |b| {
        b.iter(|| ld::unzip(black_box(&pairs)))
    });

    let small: Vec<(i32, i32)> = (0..128).map(|value| (value, value * 2)).collect();
    c.bench_function("unzip/small", |b| b.iter(|| ld::unzip(black_box(&small))));
}
