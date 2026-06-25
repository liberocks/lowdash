use criterion::{criterion_group, criterion_main, Criterion};

mod support;

fn custom_criterion() -> Criterion {
    Criterion::default().output_directory(std::path::Path::new("./report"))
}

fn all_benches(_c: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = all_benches
}
criterion_main!(benches);
