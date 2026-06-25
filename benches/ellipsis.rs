use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_ellipsis(c: &mut Criterion) {
    let input = support::long_sentence();
    c.bench_function("ellipsis", |b| {
        b.iter(|| {
            ld::ellipsis(black_box(input), black_box(48))
        })
    });
}
