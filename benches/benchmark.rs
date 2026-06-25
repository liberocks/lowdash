        use criterion::{criterion_group, criterion_main, Criterion};

        mod support;
mod common_is_collection_float;
mod common_is_floats;
mod common_random_usize;

        fn custom_criterion() -> Criterion {
            Criterion::default().output_directory(std::path::Path::new("./report"))
        }

        fn all_benches(c: &mut Criterion) {
            common_is_collection_float::benchmark_common_is_collection_float(c);
    common_is_floats::benchmark_common_is_floats(c);
    common_random_usize::benchmark_common_random_usize(c);
}

        criterion_group! {
            name = benches;
            config = custom_criterion();
            targets = all_benches
        }
        criterion_main!(benches);
