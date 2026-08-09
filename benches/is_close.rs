use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_is_close(c: &mut Criterion) {
    c.bench_function("is_close/relative", |b| {
        b.iter(|| {
            ld::is_close(
                black_box(100.0),
                black_box(100.1),
                black_box(1.0e-3),
                black_box(0.0),
            )
        })
    });
}
