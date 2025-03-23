use std::collections::HashMap;

use crate::math::number_theory::sieve_of_eratosthenes;

/// What 12-digit number do you form by concatenating the three terms in the other arithmetic
/// sequence of primes having their digits permuted?
/// 
/// Given all possible primes, we can group them by their (sorted) digits and try build an
/// arithmetic sequence. As the problem describes there are exactly two solutions with the first
/// one being known, we can filter out that case.
pub fn solve() -> u64 {
  let primes = sieve_of_eratosthenes(10_000);
  let mut groups: HashMap<String, Vec<u32>> = HashMap::new();

  // Group primes by sorted digits
  for &prime in &primes {
    let mut digits: Vec<char> = prime.to_string().chars().collect();
    digits.sort_unstable();
    let key: String = digits.into_iter().collect();
    groups.entry(key).or_default().push(prime);
  }

  // Find arithmetic sequence
  for group in groups.values() {
    if group.len() < 3 {
      continue;
    }

    for (i, &a) in group.iter().enumerate() {
      for &b in &group[(i + 1)..] {
        let c = 2 * b - a; // Enforce arithmetic sequence
        
        // Filter out the other case
        if group.contains(&c) && a != 1487 {
          return a as u64 * 100_000_000 + b as u64 * 10_000 + c as u64;
        }
      }
    }
  }

  unreachable!("There should be exactly one valid answer");
}
