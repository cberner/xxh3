use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::Rng;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("XXH3_64");
    for len in [3, 8, 16, 128, 240, 512, 1024, 4096, 65536, 1048576] {
        let mut data = vec![0u8; len];
        for x in data.iter_mut() {
            *x = rand::rng().random();
        }
        group.bench_with_input(BenchmarkId::new("crate", len), &len, |b, _| {
            b.iter(|| xxh3::hash64_with_seed(black_box(&data), black_box(1)))
        });
        group.bench_with_input(BenchmarkId::new("reference", len), &len, |b, _| {
            b.iter(|| xxhash_reference::xxh3_64(black_box(&data), black_box(1)))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
