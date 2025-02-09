// @generated
// This file is automatically generated by `build.rs`

use std::collections::BTreeMap;

use lazy_static::lazy_static;

pub mod problem_001;
pub mod problem_002;
pub mod problem_003;
pub mod problem_004;
pub mod problem_005;
pub mod problem_006;
pub mod problem_007;
pub mod problem_008;
pub mod problem_009;
pub mod problem_010;
pub mod problem_011;
pub mod problem_012;

/// A type alias for a boxed function returning a `String`, allowing for dynamic dispatch.
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
  static ref SOLVERS: BTreeMap<usize, SolveFn> = BTreeMap::from([
    (1, to_string_wrapper(problem_001::solve)),
    (2, to_string_wrapper(problem_002::solve)),
    (3, to_string_wrapper(problem_003::solve)),
    (4, to_string_wrapper(problem_004::solve)),
    (5, to_string_wrapper(problem_005::solve)),
    (6, to_string_wrapper(problem_006::solve)),
    (7, to_string_wrapper(problem_007::solve)),
    (8, to_string_wrapper(problem_008::solve)),
    (9, to_string_wrapper(problem_009::solve)),
    (10, to_string_wrapper(problem_010::solve)),
    (11, to_string_wrapper(problem_011::solve)),
    (12, to_string_wrapper(problem_012::solve)),
  ]);
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