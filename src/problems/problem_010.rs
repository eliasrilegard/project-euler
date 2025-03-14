use crate::math::number_theory::sieve_of_eratosthenes;

/// Find the sum of all the primes below two million.
///
/// Similar to Problem 7, we sieve all primes up to two million, then simply take their sum.
pub fn solve() -> u64 {
  const LIMIT: u32 = 2_000_000;
  let primes: Vec<u64> = sieve_of_eratosthenes(LIMIT)
    .iter()
    .map(|&n| n as u64)
    .collect();
  primes.iter().sum()
}
