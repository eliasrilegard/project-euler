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
pub mod problem_013;
pub mod problem_014;
pub mod problem_015;
pub mod problem_016;
pub mod problem_017;
pub mod problem_018;
pub mod problem_019;
pub mod problem_020;
pub mod problem_021;
pub mod problem_022;
pub mod problem_023;
pub mod problem_024;
pub mod problem_025;
pub mod problem_026;
pub mod problem_027;
pub mod problem_028;
pub mod problem_029;
pub mod problem_030;
pub mod problem_031;
pub mod problem_032;
pub mod problem_033;
pub mod problem_034;
pub mod problem_035;
pub mod problem_036;
pub mod problem_037;
pub mod problem_038;
pub mod problem_039;
pub mod problem_040;
pub mod problem_041;
pub mod problem_042;
pub mod problem_043;
pub mod problem_044;
pub mod problem_045;

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
    (13, to_string_wrapper(problem_013::solve)),
    (14, to_string_wrapper(problem_014::solve)),
    (15, to_string_wrapper(problem_015::solve)),
    (16, to_string_wrapper(problem_016::solve)),
    (17, to_string_wrapper(problem_017::solve)),
    (18, to_string_wrapper(problem_018::solve)),
    (19, to_string_wrapper(problem_019::solve)),
    (20, to_string_wrapper(problem_020::solve)),
    (21, to_string_wrapper(problem_021::solve)),
    (22, to_string_wrapper(problem_022::solve)),
    (23, to_string_wrapper(problem_023::solve)),
    (24, to_string_wrapper(problem_024::solve)),
    (25, to_string_wrapper(problem_025::solve)),
    (26, to_string_wrapper(problem_026::solve)),
    (27, to_string_wrapper(problem_027::solve)),
    (28, to_string_wrapper(problem_028::solve)),
    (29, to_string_wrapper(problem_029::solve)),
    (30, to_string_wrapper(problem_030::solve)),
    (31, to_string_wrapper(problem_031::solve)),
    (32, to_string_wrapper(problem_032::solve)),
    (33, to_string_wrapper(problem_033::solve)),
    (34, to_string_wrapper(problem_034::solve)),
    (35, to_string_wrapper(problem_035::solve)),
    (36, to_string_wrapper(problem_036::solve)),
    (37, to_string_wrapper(problem_037::solve)),
    (38, to_string_wrapper(problem_038::solve)),
    (39, to_string_wrapper(problem_039::solve)),
    (40, to_string_wrapper(problem_040::solve)),
    (41, to_string_wrapper(problem_041::solve)),
    (42, to_string_wrapper(problem_042::solve)),
    (43, to_string_wrapper(problem_043::solve)),
    (44, to_string_wrapper(problem_044::solve)),
    (45, to_string_wrapper(problem_045::solve)),
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
