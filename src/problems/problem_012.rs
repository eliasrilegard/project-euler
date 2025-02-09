use std::collections::HashMap;

use crate::math::number_theory::prime_factors;

/// What is the value of the first triangle number to have over five hundred divisors?
///
/// We can generate triangle numbers efficiently and keep checking until we find one which has more
/// than 500 divisors.
pub fn solve() -> u32 {
  let mut n = 1;
  let mut triangle = 1;

  while count_divisors(triangle) <= 500 {
    n += 1;
    triangle += n;
  }

  triangle
}

/// Compute the number of divisors of a number. If `N = p_1^e_1 * ... * p_k^e_k` then the total
/// number of divisors (including 1 and N itself) is `(e_1 + 1) * ... * (e_k + 1)`.
fn count_divisors(n: u32) -> u32 {
  let factors = prime_factors(n as u64);
  let mut factor_counts = HashMap::new();

  for factor in factors {
    *factor_counts.entry(factor).or_insert(0) += 1;
  }

  factor_counts.values().map(|&e| e + 1).product()
}
