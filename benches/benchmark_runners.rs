use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;
use runners;

pub fn benchmark_versions(c: &mut Criterion) {
    const INTEGER: u64 = 11701806950u64;
    let big_uint = BigUint::from(INTEGER);

    let mut group = c.benchmark_group("Runners");

    group.bench_function("version 0", |b| {
        b.iter(|| runners::version_0::run_steps_until_reaching_base(black_box(INTEGER)))
    });

    group.bench_function("version 1", |b| {
        b.iter(|| runners::version_1::run_steps_until_reaching_base(black_box(&big_uint)))
    });
}

criterion_group!(benches, benchmark_versions);
criterion_main!(benches);
