use std::collections::HashSet;

use crate::math::number_theory::sieve_of_eratosthenes;

/// Find the product of the coefficients, `|a| < 1000` and `|b| <= 1000`, for the quadratic
/// expression `n^2 + an + b` that produces the maximum number of primes for consecutive values of
/// `n`, starting with `n = 0`.
///
/// Observations:
/// - The expression starts with `n = 0`, meaning `0^2 + 0a + b` must be prime, and therefore b
///   must itself be prime.
/// - For `n = 1`, we have `1 + a + b`, which should (ideally) be prime. We can therefore filter
///   out cases where it's not.
/// - The maximum value of the polynomial will be `1000^2 + 2*1000 + 1000 = 2_001_000` This value
///   comfortably fits in an `i32`.
///
/// Start by precomputing all relevant primes. This allows for O(1) lookup later on. Iterate
/// through all possible (and relevant) combinations of `a` and `b`, keeping track of the product
/// as we go. The correctness of the algorithm is dependent on sieving all the way up to
/// `2_001_000`, but in practice the best performing polynomial only generates primes up to about
/// `n = 80`. The sieve limit can thus be reduced to `80^2 + 1000*80 + 1000 = 87_400 ≈ 90_000`.
/// Furthermore, when analyzing the algorithm at runtime we note that the maximum prime that gets
/// generated is `4961`, meaning a sieve limit of `5000` is ultimately sufficient.
pub fn solve() -> i32 {
  let limit = 1000;
  let primes: HashSet<i32> = sieve_of_eratosthenes((2 * limit * limit + limit) as u32)
    .iter()
    .map(|&p| p as i32)
    .collect();

  let mut max_count = 0;
  let mut best_product = 0;

  for &b in primes.iter().filter(|&&p| p < limit) {
    for a in 1 - b..limit {
      // a + b + 1 > 0
      if !primes.contains(&(a + b + 1)) {
        continue;
      }

      let count = consecutive_primes(a, b, &primes);
      if count > max_count {
        max_count = count;
        best_product = a * b;
      }
    }
  }

  best_product
}

fn consecutive_primes(a: i32, b: i32, primes: &HashSet<i32>) -> u32 {
  let mut n = 0;
  while primes.contains(&(n * n + a * n + b)) {
    n += 1;
  }
  n as u32
}
