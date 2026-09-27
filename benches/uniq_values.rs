use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;
use std::collections::HashMap;

pub fn benchmark_uniq_values(c: &mut Criterion) {
    let maps = support::numeric_maps(8, 256);
    let refs = support::map_refs(&maps);
    c.bench_function("uniq_values/medium", |b| {
        b.iter(|| ld::uniq_values(black_box(&refs)))
    });

    let small_maps = support::numeric_maps(4, 32);
    let small_refs = support::map_refs(&small_maps);
    c.bench_function("uniq_values/small", |b| {
        b.iter(|| ld::uniq_values(black_box(&small_refs)))
    });

    let string_maps: Vec<HashMap<usize, String>> = (0..8)
        .map(|_| {
            (0..256)
                .map(|index| (index, format!("value-{}", index % 32)))
                .collect()
        })
        .collect();
    let string_refs: Vec<_> = string_maps.iter().collect();
    c.bench_function("uniq_values/string_medium", |b| {
        b.iter(|| ld::uniq_values(black_box(&string_refs)))
    });
}
