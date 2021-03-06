use criterion::{criterion_group, criterion_main, Criterion};
use simple_bench;


fn benchmark_sled_simple(c: &mut Criterion) {
  let envs = simple_bench::setup_benchmark();
  let first = &envs[0];
  c.bench_function("simple_get", |b| b.iter(|| {
    simple_bench::benchmark_get(first);
  }));

  c.bench_function("simple_scan", |b| b.iter(|| {
    simple_bench::benchmark_scan(first);
  }));
}

criterion_group!(benches, benchmark_sled_simple);
criterion_main!(benches);