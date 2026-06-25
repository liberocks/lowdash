        use criterion::{criterion_group, criterion_main, Criterion};

        mod support;
mod common_is_collection_float;
mod common_is_floats;
mod common_random_usize;
mod common_random_usize_with_seed;
mod common_ceil_log2;
mod common_random_u64;
mod assign;
mod associate;
mod camel_case;
mod capitalize;
mod char_length;
mod chunk;
mod chunk_string;
mod clamp;
mod combination;
mod compact;
mod count;

        fn custom_criterion() -> Criterion {
            Criterion::default().output_directory(std::path::Path::new("./report"))
        }

        fn all_benches(c: &mut Criterion) {
            common_is_collection_float::benchmark_common_is_collection_float(c);
    common_is_floats::benchmark_common_is_floats(c);
    common_random_usize::benchmark_common_random_usize(c);
    common_random_usize_with_seed::benchmark_common_random_usize_with_seed(c);
    common_ceil_log2::benchmark_common_ceil_log2(c);
    common_random_u64::benchmark_common_random_u64(c);
    assign::benchmark_assign(c);
    associate::benchmark_associate(c);
    camel_case::benchmark_camel_case(c);
    capitalize::benchmark_capitalize(c);
    char_length::benchmark_char_length(c);
    chunk::benchmark_chunk(c);
    chunk_string::benchmark_chunk_string(c);
    clamp::benchmark_clamp(c);
    combination::benchmark_combination(c);
    compact::benchmark_compact(c);
    count::benchmark_count(c);
}

        criterion_group! {
            name = benches;
            config = custom_criterion();
            targets = all_benches
        }
        criterion_main!(benches);
