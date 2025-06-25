use criterion::{Criterion, criterion_group, criterion_main};
use rand::{RngCore, SeedableRng};
use std::hint::black_box;

use nanorand::Rng;
use nanorand::wyrand::WyRand;

use rand_xoshiro::Xoshiro256PlusPlus; // Using rand_xoshiro crate here

fn bench_fastrand(c: &mut Criterion) {
    c.bench_function("fastrand u64", |b| {
        b.iter(|| {
            black_box(fastrand::u64(..));
        });
    });
}

fn bench_nanorand_wyrand(c: &mut Criterion) {
    let mut rng = WyRand::new();
    c.bench_function("nanorand WyRand u64", |b| {
        b.iter(|| {
            black_box(rng.generate::<u64>());
        });
    });
}

fn bench_raw_xoshiro256(c: &mut Criterion) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(12345);
    c.bench_function("xoshiro crate Xoshiro256++ u64", |b| {
        b.iter(|| {
            black_box(rng.next_u64());
        });
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("RNG Benchmarks");
    group.measurement_time(Duration::from_secs(3)); // Optional: increase run time for stable results

    bench_fastrand(&mut group);
    bench_nanorand_wyrand(&mut group);
    bench_raw_xoshiro256(&mut group);

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
