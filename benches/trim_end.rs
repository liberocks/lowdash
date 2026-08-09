use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_trim_end(c: &mut Criterion) {
    let values = support::defaulty_int_vec(4_096);
    let cutset = [0, 1, 2];

    c.bench_function("trim_end/defaulty_int_vec", |b| {
        b.iter(|| ld::trim_end(black_box(&values), black_box(&cutset)))
    });
}
