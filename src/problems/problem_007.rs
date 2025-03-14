use crate::math::number_theory::{is_prime, sieve_of_eratosthenes};

/// What is the 10001st prime number?
///
/// The naive way to solve this problem is to successively generate each prime number until the
/// desired one is found. This approach works fine for small values but is relatively inefficient.
/// A faster (more efficient) approach involves the Sieve of Eratosthenes, which generates all
/// primes up to a given upper bound. Using the prime number theorem, we can estimate the nth prime
/// as `n * ln(n)`, which for `n = 10001` gives `92110`. Sieving up to `110_000` gives us some
/// margin.
pub fn solve() -> u32 {
  const LIMIT: u32 = 110_000;
  let primes = sieve_of_eratosthenes(LIMIT);
  primes[10_000] // Zero-indexed

  // nth_prime(10_001)
}

#[allow(dead_code)]
fn nth_prime(n: usize) -> u32 {
  let mut count = 0;
  let mut candidate = 0;
  while count < n {
    candidate += 1;
    if is_prime(candidate) {
      count += 1;
    }
  }
  candidate
}
