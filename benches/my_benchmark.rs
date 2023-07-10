use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_criterion_bench_comparing_fn::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib two", |b: &mut criterion::Bencher<'_>| b.iter(|| fibonacci_slow(black_box(20))));
}

pub fn criterion_benchmark_two(c: &mut Criterion) {
    c.bench_function("fib  one", |b: &mut criterion::Bencher<'_>| b.iter(|| fibonacci_fast(black_box(20))));
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_two);
criterion_main!(benches);

