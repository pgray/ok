use criterion::{criterion_group, criterion_main, Criterion};
use ok::{mpscbench_async, mpscbench_sync};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ok 1000", |b| b.iter(|| mpscbench_async(1000)));
    //c.bench_function("ok 10000", |b| b.iter(|| mpscbench_async(10000)));
    c.bench_function("ok 1000", |b| b.iter(|| mpscbench_sync(1000)));
    //c.bench_function("ok 10000", |b| b.iter(|| mpscbench_sync(10000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
