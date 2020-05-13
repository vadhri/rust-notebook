use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

extern crate image_utils;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("read_image_and_create_two_variants", |b| {
        b.iter(|| image_utils::read_image_and_resize(&"test-image/map.jpg".to_string(), 100, 100))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default() 
        .sample_size(10)
        .measurement_time(Duration::from_secs(15))
        .warm_up_time(Duration::from_secs(1));

    targets = criterion_benchmark
}

criterion_main!(benches);
