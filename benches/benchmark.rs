// @generated
// This file is automatically generated by `build.rs`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_euler::problems::*;

pub fn run_benchmark(c: &mut Criterion) {
  c.bench_function("problem_001", |b| b.iter(|| black_box(problem_001::solve())));
  c.bench_function("problem_002", |b| b.iter(|| black_box(problem_002::solve())));
  c.bench_function("problem_003", |b| b.iter(|| black_box(problem_003::solve())));
  c.bench_function("problem_004", |b| b.iter(|| black_box(problem_004::solve())));
  c.bench_function("problem_005", |b| b.iter(|| black_box(problem_005::solve())));
  c.bench_function("problem_006", |b| b.iter(|| black_box(problem_006::solve())));
  c.bench_function("problem_007", |b| b.iter(|| black_box(problem_007::solve())));
  c.bench_function("problem_008", |b| b.iter(|| black_box(problem_008::solve())));
  c.bench_function("problem_009", |b| b.iter(|| black_box(problem_009::solve())));
  c.bench_function("problem_010", |b| b.iter(|| black_box(problem_010::solve())));
  c.bench_function("problem_011", |b| b.iter(|| black_box(problem_011::solve())));
}

criterion_group!(benches, run_benchmark);
criterion_main!(benches);