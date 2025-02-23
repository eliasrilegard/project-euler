use std::fs;
use std::path::{Path, PathBuf};

fn main() {
  // Path to problems folder
  let problems_dir = Path::new("src/problems");

  // Read all files
  let files: Vec<PathBuf> = fs::read_dir(problems_dir)
    .expect("Failed to load problems directory")
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.path().extension().map(|e| e == "rs").unwrap_or(false))
    .map(|entry| entry.path())
    .collect();

  // Filter files to match the `problem_*.rs` pattern
  let mut problem_files_indices: Vec<String> = files
    .iter()
    .filter_map(|path| {
      let filename = path.file_name()?.to_str()?;
      if filename.starts_with("problem_") {
        Some(filename[8..filename.len() - 3].to_string()) // Extract problem number
      } else {
        None
      }
    })
    .collect();

  // Sort by index
  problem_files_indices.sort_unstable();

  // Generate the `mod.rs` file with the problem modules
  let new_mod_file_content = generate_mod_rs(&problem_files_indices);

  let mod_file_path = problems_dir.join("mod.rs");
  let current_mod_file_content = fs::read_to_string(&mod_file_path).unwrap_or_default();

  if new_mod_file_content != current_mod_file_content {
    fs::write(mod_file_path, new_mod_file_content).expect("Failed to write mod.rs");
  }

  // Generate the `benchmarks.rs` file with the problem benchmark functions
  let new_benchmark_file_content = generate_benchmarks_rs(&problem_files_indices);

  let benchmark_file_path = Path::new("benches").join("benchmark.rs");
  let current_benchmark_file_content = fs::read_to_string(&benchmark_file_path).unwrap_or_default();

  if new_benchmark_file_content != current_benchmark_file_content {
    fs::write(benchmark_file_path, new_benchmark_file_content).expect("Failed to write benchmark.rs");
  }

  // Instruct Cargo to re-run the build script if the problems directory changes
  println!("cargo::rerun-if-changed=src/problems");
  println!("cargo::rerun-if-changed=benches");
}

/// Generate the contents of `mod.rs` based on detected problem files
fn generate_mod_rs(problem_modules: &[String]) -> String {
  let mod_statements: Vec<String> = problem_modules
    .iter()
    .map(|num| format!("pub mod problem_{};", num))
    .collect();

  let insert_statements: Vec<String> = problem_modules
    .iter()
    .enumerate()
    .map(|(i, num)| format!("    ({}, to_string_wrapper(problem_{}::solve)),", i + 1, num))
    .collect();

  format!(
    "{}\n\n{}\n\n{}\n{}\n{}",
    mod_rs_start(),
    mod_statements.join("\n"),
    mod_rs_middle(),
    insert_statements.join("\n"),
    mod_rs_end()
  )
}

/// Generate the contents of `benchmarks.rs` based on detected problem files
fn generate_benchmarks_rs(problem_modules: &[String]) -> String {
  let benchmark_functions: Vec<String> = problem_modules
    .iter()
    .map(|num| {
      format!(
        "  c.bench_function(\"problem_{}\", |b| b.iter(|| black_box(problem_{}::solve())));",
        num, num
      )
    })
    .collect();

  format!(
    "{}\n{}\n{}",
    benchmark_rs_start(),
    benchmark_functions.join("\n"),
    benchmark_rs_end()
  )
}

fn mod_rs_start() -> &'static str {
  r#"// @generated
// This file is automatically generated by `build.rs`

use std::collections::BTreeMap;

use lazy_static::lazy_static;"#
}

fn mod_rs_middle() -> &'static str {
  r#"/// A type alias for a boxed function returning a `String`, allowing for dynamic dispatch.
type SolveFn = Box<dyn Fn() -> String + Send + Sync>;

/// Converts a function returning any `ToString`-implementing type into a boxed closure that
/// returns a `String`. This enables storing functions with different return types in a uniform
/// `SolveFn` type for dynamic dispatch.
fn to_string_wrapper<T>(f: fn() -> T) -> SolveFn
where
  T: ToString + 'static,
{
  Box::new(move || f().to_string())
}

lazy_static! {
  /// A lazily initiated map storing problem solvers, indexed by problem number.
  static ref SOLVERS: BTreeMap<usize, SolveFn> = BTreeMap::from(["#
}

fn mod_rs_end() -> &'static str {
  r#"  ]);
}

pub fn solve(problem_number: usize) -> String {
  SOLVERS
    .get(&problem_number)
    .map(|solver| solver())
    .unwrap_or_else(|| "Not implemented".to_string())
}

pub fn solve_all() {
  let padding_width = SOLVERS.len().to_string().len();
  for (problem_number, problem_solver) in SOLVERS.iter() {
    let result = problem_solver();
    println!("Problem {:>width$}: {}", problem_number, result, width = padding_width);
  }
}
"#
}

fn benchmark_rs_start() -> &'static str {
  r#"// @generated
// This file is automatically generated by `build.rs`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_euler::problems::*;

pub fn run_benchmark(c: &mut Criterion) {"#
}

fn benchmark_rs_end() -> &'static str {
  r#"}

criterion_group!(benches, run_benchmark);
criterion_main!(benches);
"#
}
