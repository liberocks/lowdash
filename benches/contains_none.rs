use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_contains_none(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).collect();
    let absent = [9_000, 9_001, 9_002];
    let present = [9_000, 4_000, 9_002];

    c.bench_function("contains_none/absent", |b| {
        b.iter(|| ld::contains_none(black_box(&values), black_box(&absent)))
    });

    c.bench_function("contains_none/present", |b| {
        b.iter(|| ld::contains_none(black_box(&values), black_box(&present)))
    });
}
