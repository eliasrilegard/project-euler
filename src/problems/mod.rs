use std::collections::BTreeMap;

use lazy_static::lazy_static;

pub mod problem_001;

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
