use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_euler::problems::*;

pub fn run_benchmark(c: &mut Criterion) {
  c.bench_function("problem_001", |b| b.iter(|| black_box(problem_001::solve())));
}

criterion_group!(benches, run_benchmark);
criterion_main!(benches);
