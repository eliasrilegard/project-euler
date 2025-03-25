use std::collections::HashSet;

use crate::math::number_theory::sieve_of_eratosthenes;

/// Which prime, below one-million, can be written as the sum of the most consecutive primes?
///
/// The core idea here is to use what's already been done in the previous iteration of the loop
/// when calculating the sum of consecutive primes. Here we're testing summing `primes[i..j]` and
/// checking which sums are themselves a prime.
pub fn solve() -> u32 {
  let limit = 1_000_000;
  let primes = sieve_of_eratosthenes(limit);
  let prime_set: HashSet<u32> = primes.iter().cloned().collect();

  let mut max_length = 0;
  let mut max_prime = 0;
  
  // Try using `primes[i..j]` as the summation range
  for start in 0..primes.len() {
    let mut sum = 0;
    for end in start..primes.len() {
      sum += primes[end];
    
      if sum > limit {
        break;
      }

      if end - start > max_length && prime_set.contains(&sum) {
        max_length = end - start;
        max_prime = sum;
      }
    }
  }

  max_prime
}
