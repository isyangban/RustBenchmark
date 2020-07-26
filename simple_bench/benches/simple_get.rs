use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simple_bench;


fn benchmark_sled_simple_get(c: &mut Criterion) {
  let envs = simple_bench::setupBenchmark();
  let first = envs[0];
  c.bench_function("simple_get", |b| b.iter(|| {
    simple_bench::BenchmarkGet(&first);
  }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);