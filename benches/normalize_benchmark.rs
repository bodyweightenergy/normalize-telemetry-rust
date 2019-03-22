use criterion::{criterion_group, criterion_main, Criterion};
use json_normalizer::{normalize_flatten, utils};
use rand::thread_rng;
use serde_json::json;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "normalize_flatten_rand",
        |b, &&(count, depth)| {
            let mut rng = thread_rng();
            let rand_obj = utils::rand_json_obj(&mut rng, count, depth);

            b.iter(|| normalize_flatten(&rand_obj));
        },
        &[(10, 1), (10, 3), (10, 5), (1000, 1), (100, 1), (100, 2)],
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
