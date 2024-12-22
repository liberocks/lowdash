use criterion::{criterion_group, criterion_main, Criterion};

mod find;

use find::*;

fn custom_criterion() -> Criterion {
    Criterion::default().output_directory(std::path::Path::new("./report"))
}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = benchmark_find_i32, benchmark_find_person,benchmark_find_float
}
criterion_main!(benches);
